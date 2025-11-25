/// Listens on UDP 9999 and replies to any packet with its ID string.
use tokio::{
    fs::File,
    io::AsyncWriteExt,
    net::{TcpListener, UdpSocket},
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let id = "receiver-online";
    let sock = UdpSocket::bind("0.0.0.0:9999").await?;
    let tcp_port = 9998;
    let mut buf = [0u8; 1024];

    let (_n, addr) = sock.recv_from(&mut buf).await?;
    let data = String::from_utf8_lossy(&buf);
    println!("{data}");
    let _ = sock
        .send_to(format!("{id};{tcp_port}").as_bytes(), addr)
        .await?;
    sock.connect(addr);

    let (_n, addr) = sock.recv_from(&mut buf).await?;
    let data = String::from_utf8_lossy(&buf);
    let mut data = data.split(';');
    let size = data.next().unwrap().parse::<u64>().unwrap();
    let name = data.next().unwrap();

    let mut f = File::create(name).await?;

    let (n, addr) = sock.recv_from(&mut buf).await?;
    let tcp = TcpListener::bind("0.0.0.0:9998").await?;
    f.write_all(&buf).await.unwrap();

    Ok(())
}
