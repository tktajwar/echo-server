use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
	let (mut socket, _) = listener.accept().await?;

	tokio::spawn(async move {
	    println!("-- New Connection --");
	    let mut buf = [0; 1024];
	    loop {
		let bytes_read = match socket.read(&mut buf).await {
		    Ok(0) => {
			println!("-- Connection Closed --");
			return
		    },
		    Ok(n) => n,
		    Err(err) => {
			eprintln!("Error from reading socket: {:?}", err);
			0
		    },
		};
		println!("{:?}", &buf[..bytes_read]);

		if let Err(err) = socket.write_all(&buf[..bytes_read]).await {
		    eprintln!("Error from writing to socket: {:?}", err);
		}
	    }
	});
    }
}
