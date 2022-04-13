use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::Duration;

use afire::{internal::http, prelude::*};
use lazy_static::lazy_static;
use url::Url;

lazy_static! {
    static ref ROOT_STORE: rustls::RootCertStore = {
        let mut root_store = rustls::RootCertStore::empty();
        root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));
        root_store
    };
    static ref CLIENT_CONFIG: rustls::ClientConfig = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(ROOT_STORE.to_owned())
        .with_no_client_auth();
}

struct App;

impl Middleware for App {
    fn post(&self, req: Request, _res: Response) -> MiddleResponse {
        let timeout = Some(Duration::from_secs(5));

        let url = match req.path.strip_prefix("/p/") {
            Some(i) => i,
            None => return MiddleResponse::Continue,
        };
        let url = Url::parse(url).unwrap();
        // Disallow localhost requests
        // match url.host_str() {
        //     Some("localhost") | Some("127.0.0.1") => panic!("Localhost is off limits :p"),
        //     _ => {}
        // }

        let http_send = gen_request(&req, &url);
        let mut buff = match url.scheme() {
            "http" => request(&url, &http_send, timeout),
            "https" => unimplemented!(),
            _ => panic!("Unsupported URL Scheme"),
        };
        strip_buff(&mut buff);

        // Parse Response
        let stream_string = String::from_utf8_lossy(&buff);
        let headers = http::get_request_headers(&stream_string)
            .into_iter()
            .filter(|x| x.name != "Content-Length")
            .collect();
        let status = http_status(&stream_string);
        let body = http::get_request_body(&buff);

        // Send Response
        MiddleResponse::Add(Response::new().status(status).bytes(body).headers(headers))
    }
}

fn request(url: &Url, send: &[u8], timeout: Option<Duration>) -> Vec<u8> {
    // Open request socket
    let mut stream = TcpStream::connect(url.socket_addrs(|| url.port()).unwrap()[0]).unwrap();
    stream.set_read_timeout(timeout).unwrap();
    stream.set_write_timeout(timeout).unwrap();

    // Send Data
    stream.write_all(&send).unwrap();

    // Get response
    let mut buff = vec![0; 1024];
    loop {
        match stream.read(&mut buff) {
            Err(_) => break,
            Ok(0) => break,
            Ok(i) => buff.extend(vec![0; i]),
        }
    }
    buff
}

// fn request_tls(url: &Url, send: &[u8], timeout: Option<Duration>) -> Vec<u8> {
//     // Open request socket
//     let mut stream = rustls::ClientConnection::new(
//         Arc::new(*CLIENT_CONFIG),
//         url.host_str().unwrap().try_into().unwrap(),
//     )
//     .unwrap();
//     let mut stream = stream.writer();
//     // url.socket_addrs(|| url.port()).unwrap()[0]
//     // stream.set_read_timeout(timeout).unwrap();
//     // stream.set_write_timeout(timeout).unwrap();
//
//     // Send Data
//     stream.write_all(&send).unwrap();
//
//     // Get response
//     let mut buff = vec![0; 1024];
//     loop {
//         match stream.rea(&mut buff) {
//             Err(_) => break,
//             Ok(0) => break,
//             Ok(i) => buff.extend(vec![0; i]),
//         }
//     }
//     buff
// }

fn gen_request(req: &Request, url: &Url) -> Vec<u8> {
    let mut headers = req
        .headers
        .iter()
        .filter(|x| x.name != "Host")
        .collect::<Vec<_>>();
    let host_header = Header::new("Host", url.host_str().unwrap());
    headers.insert(0, &host_header);
    let mut http_send = format!(
        "{} {} HTTP/1.1\r\n{}\r\n\r\n",
        req.method,
        url.path(),
        headers
            .iter()
            .map(|x| x.to_string() + "\r\n")
            .collect::<String>()
    )
    .as_bytes()
    .to_vec();
    http_send.extend(req.body.clone());

    http_send
}

fn main() {
    let mut server = Server::new("localhost", 8080);
    App.attach(&mut server);
    server.start_threaded(64);
}

fn strip_buff(buff: &mut Vec<u8>) {
    while buff.last() == Some(&b'\0') {
        buff.pop();
    }
}

fn http_status(str: &str) -> u16 {
    let mut parts = str.splitn(3, " ");
    return parts.nth(1).unwrap().parse().unwrap();
}
