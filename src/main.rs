use std::time::Duration;
use std::{borrow::Cow, io::Read};

use afire::prelude::*;
use ureq::{AgentBuilder, Error};
use url::Url;

const BLOCKED_HEADERS: &[&str] = &[
    "transfer-encoding",
    "connection",
    "content-security-policy",
    "referrer-policy",
];

fn url(str: &str) -> Url {
    if str.starts_with("http") {
        return Url::parse(str).unwrap();
    }

    Url::parse(&format!("https://{str}")).unwrap()
}

fn main() {
    let mut server = Server::<()>::new("localhost", 8080);
    // ServeStatic::new("./web/static").attach(&mut server);

    server.route(Method::ANY, "**", |req| {
        let addr = if let Some(i) = req.header("Referer") {
            let url = url(&i);
            let path = &url.path()[1..];
            Cow::Owned(format!("{path}/{}", &req.path[1..]))
        } else {
            Cow::Borrowed(&req.path[1..])
        };

        let url = url(&addr);
        println!("[HANDLING] `{}`", url);

        // TODO: Disallow loopback requests

        // Make agent
        let agent = AgentBuilder::new().redirects(0).build();

        // Make request to real server
        let mut res = agent
            .request(&req.method.to_string(), url.as_str())
            .timeout(Duration::from_secs(5));

        // Add headers to server request
        for i in req.headers {
            res = res.set(&i.name, &i.value);
        }

        if let Some(i) = url.host_str() {
            res = res.set("Host", i);
        }

        // Send request
        let res = match res.send_bytes(&req.body) {
            Ok(i) => i,
            Err(Error::Status(_, i)) => i,
            Err(e) => return Response::new().status(500).text(e),
        };

        // Make client response
        let mut headers = Vec::new();
        for i in res
            .headers_names()
            .iter()
            .filter(|x| !BLOCKED_HEADERS.contains(&x.to_ascii_lowercase().as_str()))
        {
            let mut header = Header::new(i, res.header(i).unwrap());
            if header.name == "Location" {
                header.value = format!("/p/{}", header.value);
                continue;
            }

            headers.push(header);
        }
        let resp = Response::new()
            .status(res.status())
            .headers(headers)
            .header("referrer-policy", "unsafe-url");

        // Send Response
        // TODO: Update afire for streaming responces
        let mut buff = Vec::new();
        res.into_reader().read_to_end(&mut buff).unwrap();
        resp.bytes(buff)
    });

    server.start_threaded(64).unwrap();
}

// == TODOS ==
// - Analytis in a sqlite db and graphs on the homepage
// - Said homepage will be a search like box with a grayed out `https://`
//   For the user to type an address into, if its not a valid address, ddg search it.
