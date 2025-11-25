/// Broadcasts a packet to UDP 9999 and prints every response source IP.
use tokio::{
    fs::{File, read_to_string},
    io::AsyncWriteExt,
    net::{TcpSocket, TcpStream, UdpSocket},
    time,
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    sock.set_broadcast(true)?;
    sock.send_to(b"DISCOVER", "255.255.255.255:9999").await?;

    let mut buf = [0u8; 1024];
    let end = time::Instant::now() + time::Duration::from_secs(2);

    let addr: Option<(std::net::SocketAddr, usize)> = loop {
        if time::Instant::now() > end {
            break None;
        }
        println!("Probing...");
        if let Ok((n, addr)) = sock.try_recv_from(&mut buf) {
            let msg = String::from_utf8_lossy(&buf).to_string();
            let mut split = msg.split(';');
            let id = split.next().unwrap();
            let port: usize = split.next().unwrap().parse().unwrap();
            break Some((addr, port));
        }
        time::sleep(time::Duration::from_millis(50)).await;
    };
    let file = File::open("/home/levi/Coding/rust/send/example.txt").await?;

    if let Some((addr, port)) = addr {
        let mut tcp = TcpStream::connect(format!("{addr}:{port}")).await?;
        let s = read_to_string("/home/levi/Coding/rust/send/example.txt").await?;
        tcp.write_all(s.as_bytes()).await.unwrap();
        println!("Found reciever on addr: {}", addr);
        sock.set_broadcast(false)?;

        let size = file.max_buf_size();
        sock.send(format!("{size};example.txt;").as_bytes()).await?;
    }

    Ok(())
}
