use std::io::Read;
use std::time::Duration;

use afire::{extension::ServeStatic, prelude::*};
use ureq::Error;
use url::Url;

const BLOCKED_HEADERS: &[&str] = &["transfer-encoding", "connection"];

fn main() {
    let mut server = Server::<()>::new("localhost", 8080);
    ServeStatic::new("./web/static").attach(&mut server);

    server.route(Method::ANY, "/p/**", |req| {
        let url = Url::parse(&req.path.strip_prefix("/p/").unwrap()).expect("Invalid URL");

        // Disallow localhost requests
        match url.host_str() {
            Some("localhost") | Some("127.0.0.1") => panic!("Localhost is off limits :p"),
            _ => {}
        }

        // Make request to real server
        let mut res =
            ureq::request(&req.method.to_string(), url.as_str()).timeout(Duration::from_secs(5));

        // Add headers to server reques
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
            Err(e) => panic!("{}", e),
        };

        // Make client respose
        let mut headers = Vec::new();
        for i in res
            .headers_names()
            .iter()
            .filter(|x| !BLOCKED_HEADERS.contains(&x.as_str()))
        {
            headers.push(Header::new(i, res.header(i).unwrap()));
        }
        let resp = Response::new().status(res.status()).headers(headers);

        // Send Response
        let mut buff = Vec::new();
        res.into_reader().read_to_end(&mut buff).unwrap();
        resp.bytes(buff)
    });

    // server.start_threaded(64);
    server.start().unwrap();
}
