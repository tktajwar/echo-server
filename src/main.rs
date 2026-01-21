use tokio::io::{AsyncReadExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
	let (mut socket, _) = listener.accept().await?;

	tokio::spawn(async move {
	    let mut buf = [0; 1024];
	    let bytes_read = match socket.read(&mut buf).await {
		Ok(n) => n,
		Err(err) => {
		    eprintln!("Error from reading socket: {:?}", err);
		    0
		},
	    };
	    println!("{:?}", &buf[..bytes_read]);
	});
    }
}
