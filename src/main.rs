use std::{net::{TcpListener, TcpStream}, io};
use httparse;
use async_std::task;

async fn parse_http(ring: &rio::Rio, a: &TcpStream, b: &TcpStream) -> io::Result<()> {

    let  msg = "HTTP/1.1 200 OK
Content-Length: 1
Content-Type: text/plain

a";

    let buf = vec![0_u8; 512];
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);

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
    ring.send(b, &msg).await?;
    return io::Result::Ok(());
}

fn main() -> io::Result<()> {
    let ring = rio::new()?;
    let acceptor = TcpListener::bind("127.0.0.1:6666")?;

    task::block_on(async {
        // kernel 5.5 and later support TCP accept
        loop {
            let stream = ring.accept(&acceptor).await?;
            parse_http(&ring, &stream, &stream).await.unwrap();
        }
    })
}
