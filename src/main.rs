use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use afire::{internal::http, prelude::*};
use url::Url;

struct App;

impl Middleware for App {
    fn post(&self, req: Request, _res: Response) -> MiddleResponse {
        let timeout = Duration::from_secs(5);

        let url = match req.path.strip_prefix("/p/") {
            Some(i) => i,
            None => return MiddleResponse::Continue,
        };
        let url = Url::parse(url).unwrap();
        // match url.host_str() {
        //     Some("localhost") | Some("127.0.0.1") => panic!("Localhost is off limits :p"),
        //     _ => {}
        // }

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
        http_send.extend(req.body);

        println!("{}", String::from_utf8_lossy(&http_send));

        let mut stream = TcpStream::connect(url.socket_addrs(|| url.port()).unwrap()[0]).unwrap();
        stream.set_read_timeout(Some(timeout)).unwrap();
        stream.set_write_timeout(Some(timeout)).unwrap();

        stream.write_all(&http_send).unwrap();
        println!("Sent");

        let mut buff = vec![0; 1024];
        loop {
            match stream.read(&mut buff) {
                Err(_) => break,
                Ok(0) => break,
                Ok(i) => buff.extend(vec![0; i]),
            }
        }
        strip_buff(&mut buff);
        println!("Recv {}", buff.len());

        let stream_string = String::from_utf8_lossy(&buff);
        let headers = http::get_request_headers(&stream_string)
            .into_iter()
            .filter(|x| x.name != "Content-Length")
            .collect();
        let status = http_status(&stream_string);
        let body = http::get_request_body(&buff);
        dbg!(String::from_utf8_lossy(&body));

        MiddleResponse::Add(Response::new().status(status).bytes(body).headers(headers))
    }
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
