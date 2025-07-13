// Network communication layer

pub mod ssh;
pub mod tcp;
pub mod protocol;

use async_trait::async_trait;

#[async_trait]
pub trait Connection: Send + Sync {
    async fn connect(&mut self) -> crate::error::Result<()>;
    async fn send(&mut self, data: &[u8]) -> crate::error::Result<usize>;
    async fn receive(&mut self, buf: &mut [u8]) -> crate::error::Result<usize>;
    async fn close(&mut self) -> crate::error::Result<()>;
}