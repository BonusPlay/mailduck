use crate::smtp::commands::command::Command;

use async_trait::async_trait;
use lazy_static::lazy_static;
use log::info;
use regex::Regex;
use tokio::{net::TcpStream, io::AsyncWriteExt};
use color_eyre::Report;

// https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.1.1

#[derive(Debug)]
pub struct SmtpParam {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct SmtpMailFrom {
    pub reverse_path: String,
    pub params: Vec<SmtpParam>,
}

#[async_trait]
impl Command for SmtpMailFrom {
    fn from_str(cmd: &str) -> Result<Box<Self>, Report> {
        info!("parsing mail from");

        lazy_static!{
            // TODO: this is not a valid RFC5321 regex I couldn't be bothered to create a real one
            static ref RE: Regex = Regex::new(r"(MAIL\ FROM):\<(.*?)\>").unwrap();
            // MAIL\ FROM:\<(.*?)\>\ (\ ?([^=\r\n\ ]+)=([^\r\n\ ]*))+?
        }

        // TODO: handle unwrap properly
        let captures = RE.captures(cmd).unwrap();

        // TODO: parse parameters
        
        Ok(Box::new(SmtpMailFrom{
            reverse_path: captures.get(0).unwrap().as_str().into(),
            params: vec!(),
        }))
    }

    async fn handle(&self, socket: &mut TcpStream) -> Result<(), Report> {
        info!("handling mail from");

        socket.write_all("250-OK\r\n".as_bytes()).await?;

        Ok(())
    }
}
