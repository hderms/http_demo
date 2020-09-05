use std::{net::{TcpListener, TcpStream}, io};
use httparse;
use async_std::task;
use rio::Config;
use std::sync::Arc;

async fn parse_http(ring: rio::Rio, a: &TcpStream) -> io::Result<()> {

    let  msg = "HTTP/1.1 200 OK
Content-Length: 1
Content-Type: text/plain

a";

    let  mut buf = [0_u8; 512];
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

    let buf = &mut buf[..];
    loop {
        let read_bytes = ring.read_at(a, &buf, 0).await?;
        let buf = &buf[..read_bytes];
        let res = req.parse(buf).unwrap();
        if res.is_partial() {
            continue;
        } else {
            break;
        }
    }
    ring.send(a, &msg).await?;
    return io::Result::Ok(());
}

#[async_std::main]
async fn main() -> io::Result<()> {
    let ring = rio::new()?;

    let acceptor = TcpListener::bind("127.0.0.1:6666").unwrap();

    loop {
        let ring = ring.clone();
        let  stream = ring.accept(&acceptor).await.unwrap();
        task::spawn(async move  {
            parse_http(ring, &stream).await.unwrap();
        });
    }

}


