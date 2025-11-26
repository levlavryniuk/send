use std::env::args;

/// Broadcasts a packet to UDP 9999 and prints every response source IP.
use tokio::{
    fs::{File, read_to_string},
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpSocket, TcpStream, UdpSocket},
    time,
};

pub async fn send(file_name: &str, to: &str) -> tokio::io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    sock.set_broadcast(true)?;
    sock.send_to(b"TRANSFER", "255.255.255.255:9999").await?;

    let mut buf = [0u8; 1024];
    let end = time::Instant::now() + time::Duration::from_secs(2);

    let addr: Option<(std::net::SocketAddr, u16)> = loop {
        if time::Instant::now() > end {
            break None;
        }
        println!("Probing...");
        if let Ok((n, addr)) = sock.try_recv_from(&mut buf) {
            let msg = String::from_utf8_lossy(&buf[..n]).to_string();
            let mut split = msg.split(';');
            let id = split.next().unwrap();
            let port: u16 = split.next().unwrap().parse().unwrap();
            break Some((addr, port));
        }
        time::sleep(time::Duration::from_millis(50)).await;
    };
    let fp = file_name;
    let file_name = file_name.rsplit('/').next().unwrap();

    if let Some((mut addr, port)) = addr {
        addr.set_port(port);
        let size = tokio::fs::metadata(fp).await?.len();
        let mut tcp = TcpStream::connect(addr).await?;
        let mut file = File::open(file_name).await?;
        println!("Found reciever on addr: {}", addr);
        println!("Sending {file_name}...");

        tcp.write_u32_le(file_name.len() as u32).await?;
        let mut file_buf = vec![0u8; size as usize];
        tcp.write_u64_le(size as u64).await?;
        tcp.write(file_name.as_bytes()).await?;

        loop {
            let bytes = file.read(&mut file_buf).await?;
            println!("Sending {bytes} bytes...");
            if bytes == 0 {
                break;
            }
            tcp.write_all(&file_buf[..bytes]).await?;
        }
    }

    Ok(())
}
