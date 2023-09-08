use std::time::Duration;
use std::time::Instant;

use afire::{internal::encoding, prelude::*, route::RouteContext};
use ureq::{AgentBuilder, Error};
use url::{ParseError, Url};

use crate::app::App;
use crate::proxy::headers::{transform_header_c2s, transform_header_s2c};

mod headers;
mod rewrite;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::ANY, "/p/{path}", |ctx| {
        let raw_url = encoding::url::decode(ctx.param_idx(0));
        let mut url = Url::parse(&raw_url);
        if let Err(ParseError::RelativeUrlWithoutBase) = url {
            url = Url::parse(&format!("https://{}", raw_url));
        }
        let mut url = url.context("Invalid URL")?;
        if !ctx.req.query.is_empty() {
            url.set_query(Some(&ctx.req.query.to_string()[1..]));
        }

        #[cfg(debug_assertions)]
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
        for i in ctx
            .req
            .headers
            .iter()
            .filter_map(|x| transform_header_c2s(x))
        {
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
            .log_request(&ctx.req, &url, res.status(), time.elapsed())?;

        // Make client response
        let headers = res
            .headers_names()
            .iter()
            .map(|x| Header::new(x, res.header(x).unwrap()))
            .filter_map(|x| transform_header_s2c(x, &url))
            .collect::<Vec<_>>();

        // Optionally rewrite HTML
        let status = res.status();
        if res
            .header("Content-Type")
            .unwrap_or_default()
            .starts_with("text/html")
        {
            let body = res.into_string()?;
            let body = rewrite::rewrite(&body, &url)?;
            ctx.bytes(body);
            // ctx.modifier(|res| res.headers.retain(|i| i.name != HeaderName::ContentType));
            // ctx.header((HeaderName::ContentType, "text/html; charset=utf-8"));
        } else {
            ctx.stream(res.into_reader());
        }

        ctx.status(status)
            .headers(headers)
            .header(("Referrer-Policy", "unsafe-url"))
            .send()?;
        Ok(())
    });
}

// TODO: header transformer
// - Location
// - Referrer
