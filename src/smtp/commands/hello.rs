use crate::smtp::commands::command::Command;
use crate::smtp::error::SmtpError;

use async_trait::async_trait;
use log::{info, warn};
use tokio::{net::TcpStream, io::AsyncWriteExt};
use color_eyre::Report;

// https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.1.1

#[derive(Debug)]
pub struct SmtpHello {
    pub host: String,
}

#[async_trait]
impl Command for SmtpHello {
    fn from_str(cmd: &str) -> Result<Box<Self>, Report> {
        info!("parsing helo");

        let parts: Vec<&str> = cmd.split(" ").collect();
        if parts.len() != 2 {
            warn!("invalid amount of arguments passed to create SmtpHello {:?}", parts);
            return Err(SmtpError::InvalidRequest.into());
        };

        {
            Ok(Box::new(SmtpHello{
                host: parts[1].to_string(),
            }))
        }
    }

    async fn handle(&self, socket: &mut TcpStream) -> Result<(), Report> {
        info!("handling helo");

        socket.write_all(format!("250-mail.bonus.p4 greets {}\r\n", self.host).as_bytes()).await?;

        Ok(())
    }
}
