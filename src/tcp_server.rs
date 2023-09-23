pub mod tcp_server {

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};



pub async fn start_tcp_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // Em um loop, ler dados do socket e escrever de volta.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket fechado
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("falha ao ler do socket; err = {:?}", e);
                        return;
                    }
                };

                // Escrever os dados de volta
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("falha ao escrever no socket; err = {:?}", e);
                    return;
                }
            }
            
        });
    }
}

}    