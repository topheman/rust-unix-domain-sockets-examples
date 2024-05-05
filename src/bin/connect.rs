use std::io;
use tokio::io::Interest;
use tokio::net::UnixListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = UnixListener::bind("mysocket").unwrap();
    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                println!("new client!");
                let ready = stream
                    .ready(Interest::READABLE | Interest::WRITABLE)
                    .await?;
                if ready.is_readable() {
                    let mut data = vec![0; 1024];
                    // Try to read data, this may still fail with `WouldBlock`
                    // if the readiness event is a false positive.
                    match stream.try_read(&mut data) {
                        Ok(n) => {
                            println!("read {} bytes", n);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
