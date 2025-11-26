use core::panic;

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, UdpSocket},
};

pub async fn activate_reciever() -> tokio::io::Result<()> {
    let id = "receiver-online";
    let sock = UdpSocket::bind("0.0.0.0:9999").await?;
    let tcp_port = 9998;
    let mut buf = [0u8; 1024];

    loop {
        let (n, addr) = sock.recv_from(&mut buf).await?;
        let message = String::from_utf8_lossy(&buf[..n]);
        match message.as_ref() {
            "DISCOVER" => {
                println!("{addr} says Hi");
                sock.send_to(format!("{id};{tcp_port}").as_bytes(), addr)
                    .await?;
            }
            "TRANSFER" => {
                println!("{addr} wants to send files");
                let res = sock
                    .send_to(format!("{id};{tcp_port}").as_bytes(), addr)
                    .await;

                if let Ok(_) = res {
                    println!("Switching over to TCP");
                    switch_protocols().await?;
                };
            }
            _ => {
                panic!("What do you want from me, stranger?")
            }
        };

        println!("Operation success, listening...");
    }
}
async fn switch_protocols() -> tokio::io::Result<()> {
    let tcp = TcpListener::bind("0.0.0.0:9998").await.unwrap();
    let (mut stream, ..) = tcp.accept().await.unwrap();
    let name_len = stream.read_u32_le().await?;
    let file_len = stream.read_u64_le().await?;
    let mut file_name = vec![0u8; name_len as usize];
    stream.read_exact(&mut file_name).await?;
    let file_name = String::from_utf8_lossy(&file_name);
    dbg!(&file_name);
    let mut remaining = file_len as usize;
    let mut file_buf = vec![0u8; 8192];
    let mut file = File::create(format!("/home/levi/Downloads/{file_name}")).await?;
    loop {
        if remaining == 0 {
            break;
        }
        let read_bytes = stream.read(&mut file_buf).await?;
        if read_bytes == 0 && remaining != 0 {
            panic!("Why stopped, noob?");
        }
        file.write_all(&file_buf[..read_bytes]).await?;
        remaining -= read_bytes;
        println!("Got {read_bytes}, remaining:{remaining}");
    }
    stream.shutdown().await
}
