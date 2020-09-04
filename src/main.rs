use std::{
    io::self,
    net::{TcpListener, TcpStream},
};
use httparse;

async fn parse_http(ring: &rio::Rio, a: &TcpStream, b: &TcpStream) -> io::Result<()> {
    let buf = vec![0_u8; 512];
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    
    loop {
        let read_bytes = ring.read_at(a, &buf, 0).await?;
        let buf = &buf[..read_bytes];
        let msg = "HTTP/1.1 200 OK
Content-Length: 1
Content-Type: text/plain

a";
        req.parse(buf).unwrap();
        ring.write_at(b, &msg, 0).await?;
    }
}

fn main() -> io::Result<()> {
    let ring = rio::new()?;
    let acceptor = TcpListener::bind("127.0.0.1:6666")?;

    extreme::run(async {
        // kernel 5.5 and later support TCP accept
        loop {
            let stream = ring.accept(&acceptor).await?;
            dbg!(parse_http(&ring, &stream, &stream).await.unwrap());
        }
    })
}