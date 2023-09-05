use std::time::Duration;
use std::{borrow::Cow, time::Instant};

use afire::{internal::encoding, prelude::*, route::RouteContext};
use ureq::{AgentBuilder, Error};
use url::{ParseError, Url};

use crate::{app::App, BLOCKED_HEADERS};

pub fn attach(server: &mut Server<App>) {
    server.route(Method::ANY, "/p/{path}", |ctx| {
        let raw_url = encoding::url::decode(ctx.param_idx(0));
        let mut url = Url::parse(&raw_url);
        if let Err(ParseError::RelativeUrlWithoutBase) = url {
            url = Url::parse(&format!("https://{}", raw_url));
        }
        let url = url.context("Invalid URL")?;

        println!("[HANDLING] `{}`", url);

        // Disallow localhost requests
        if let Some("localhost") | Some("127.0.0.1") = url.host_str() {
            return Ok(ctx
                .status(500)
                .text("Localhost is off limits. Nice try.")
                .send()?);
        }

        // Make request to real server
        let timeout = ctx.app().config.timeout_ms;
        let agent = AgentBuilder::new()
            .redirects(0)
            .timeout(Duration::from_millis(timeout))
            .build();
        let mut res = agent.request(&ctx.req.method.to_string(), url.as_str());

        // Add headers to server request
        for i in ctx.req.headers.iter() {
            res = res.set(&i.name.to_string(), &i.value);
        }

        if let Some(i) = url.host_str() {
            res = res.set("Host", i);
        }

        // Send request
        let time = Instant::now();
        let res = match res.send_bytes(&ctx.req.body) {
            Ok(i) => i,
            Err(Error::Status(_, i)) => i,
            Err(e) => {
                return Ok(ctx
                    .status(500)
                    .text(format!("Transport error: {e}"))
                    .send()?)
            }
        };

        // Log request for analytical purposes
        // how devious of me ^w^
        ctx.app()
            .analytics
            .log_request(&ctx.req, url.as_str(), res.status(), time.elapsed())?;

        // Make client response
        let headers = res
            .headers_names()
            .iter()
            .filter(|x| !BLOCKED_HEADERS.contains(&x.to_ascii_lowercase().as_str()))
            .map(|i| {
                let mut header = Header::new(i, res.header(i).unwrap());
                if header.name == HeaderName::Location {
                    header.value = Cow::Owned(format!("/p/{}", header.value));
                }
                header
            })
            .collect::<Vec<_>>();

        ctx.status(res.status())
            .headers(headers)
            .header(("Referrer-Policy", "unsafe-url"))
            .stream(res.into_reader())
            .send()?;
        Ok(())
    });
}
