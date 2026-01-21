use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
	let (mut socket, _) = listener.accept().await?;

	tokio::spawn(async move {
	    let mut buf = [0; 1024];
	    let bytes_read = match socket.read(&mut buf).await {
		Ok(0) => return,
		Ok(n) => n,
		Err(err) => {
		    eprintln!("Error from reading socket: {:?}", err);
		    0
		},
	    };
	    println!("{:?}", &buf[..bytes_read]);

	    let msg = format!(
		"HTTP/1.1 200 OK\r\n\
		 Content-Type: text/plain\r\n\
		 Content-Length: {}\r\n\
		 \r\n\
		 OK",
		"OK".len()
	    );


	    if let Err(err) = socket.write_all(msg.as_bytes()).await {
		eprintln!("Error from writing to socket: {:?}", err);
	    }
	});
    }
}
