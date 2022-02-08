use color_eyre::Report;
use smtp::commands::{SmtpHello, SmtpMailFrom, Command};
use tokio::{net::{TcpListener, TcpStream}, io::AsyncWriteExt};
use log::{info, error, warn};
use tokio::io::AsyncReadExt;
use std::str;

mod smtp;

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;
    env_logger::init();

    info!("mailduck starting...");

    let listener = TcpListener::bind("127.0.0.1:1337").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let peer_addr = socket.peer_addr()?;
        info!("client connected: {:?}", peer_addr);
        tokio::spawn(async move {
            handle_connection(&mut socket).await;
            info!("client disconnected: {:?}", peer_addr);
        });
    }
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    Ok(())
}

async fn handle_connection(socket: &mut TcpStream) -> Result<(), Report> {

    // wait for socket to be wrtable
    socket.writable().await?;

    // start exchange
    socket.write_all("220 mail.bonus.p4 mailduck ready".as_bytes()).await?;

    // wait for socket to be readable
    socket.readable().await?;

    loop {
        let mut buf = [0; 1024];

        let n = {
            let n = match socket.read(&mut buf).await {
                Ok(0) => {
                    info!("socket closed");
                    return Ok(())
                },
                Ok(n) => {
                    info!("read {:?} bytes", n);
                    Ok(n)
                },
                Err(e) => {
                    error!("failed to read from socket, err = {:?}", e);
                    Err(e)
                }
            }?;
            n
        };

        for cmd in str::from_utf8(&buf[0..n])?.split("\r\n") {
            if cmd.is_empty() {
                continue
            }

            // it's ugly but idgaf
            // tried to use streams, but async + streams == :wtf:
            match cmd.split(" ").nth(0) {
                Some(part) => {
                    match part {
                        "HELO" => {
                            let msg: SmtpHello = *(Command::from_str(cmd)?);
                            msg.handle(socket).await?;
                        },
                        "MAIL" => {
                            let msg: SmtpMailFrom = *(Command::from_str(cmd)?);
                            msg.handle(socket).await?;
                        },
                        _ => {
                            warn!("unknown cmd {:X?}", cmd);
                            socket.write_all("UNKNOWN COMMAND".as_bytes()).await?;
                        }
                    }
                },
                None => {
                    warn!("Newline not found in cmd {:X?}", cmd);
                },
            };
        };
    }
}
