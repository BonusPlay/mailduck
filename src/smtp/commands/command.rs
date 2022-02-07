use tokio::net::TcpStream;
use async_trait::async_trait;
use color_eyre::Report;

#[async_trait]
pub trait Command {
    fn from_str(cmd: &str) -> Result<Box<Self>, Report>;
    async fn handle(&self, socket: &mut TcpStream) -> Result<(), Report>;
}
