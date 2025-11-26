use std::net::SocketAddr;
use tokio::time;

use tokio::net::UdpSocket;

#[derive(Debug)]
pub struct Reciever {
    pub id: String,
    pub addr: SocketAddr,
    pub tcp_port: u16,
}
pub async fn list_recievers() -> tokio::io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    sock.set_broadcast(true)?;
    sock.send_to(b"DISCOVER", "255.255.255.255:9999").await?;

    let mut buf = [0u8; 1024];
    let end = time::Instant::now() + time::Duration::from_secs(2);
    let mut recievers: Vec<Reciever> = Vec::new();
    println!("Probing for 2 seconds...");

    loop {
        if time::Instant::now() > end {
            break;
        }
        if let Ok((n, addr)) = sock.try_recv_from(&mut buf) {
            let msg = String::from_utf8_lossy(&buf[..n]).to_string();
            let mut split = msg.split(';');
            let id = split.next().unwrap();
            let port: u16 = split.next().unwrap().parse().unwrap();
            recievers.push(Reciever {
                id: id.to_string(),
                tcp_port: port,
                addr: addr,
            });
        }
        time::sleep(time::Duration::from_millis(50)).await;
    }
    for rec in recievers {
        println!("{} from {}", rec.id, rec.addr)
    }
    Ok(())
}
