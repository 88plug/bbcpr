use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, info};

use crate::error::{BbcprError, Result};
use crate::network::Connection;

pub struct TcpConnection {
    address: SocketAddr,
    stream: Option<TcpStream>,
    window_size: usize,
}

impl TcpConnection {
    pub fn new(address: SocketAddr, window_size: usize) -> Self {
        Self {
            address,
            stream: None,
            window_size,
        }
    }
    
    fn configure_socket(stream: &TcpStream) -> Result<()> {
        use socket2::Socket;
        use std::os::unix::io::AsRawFd;
        
        let sock = unsafe {
            Socket::from_raw_fd(stream.as_raw_fd())
        };
        
        // Set TCP_NODELAY for low latency
        sock.set_nodelay(true)
            .map_err(|e| BbcprError::Network(format!("Failed to set TCP_NODELAY: {}", e)))?;
        
        // Set send/receive buffer sizes
        sock.set_send_buffer_size(1024 * 1024)
            .map_err(|e| BbcprError::Network(format!("Failed to set send buffer: {}", e)))?;
        
        sock.set_recv_buffer_size(1024 * 1024)
            .map_err(|e| BbcprError::Network(format!("Failed to set recv buffer: {}", e)))?;
        
        // Don't drop the socket, we're just borrowing it
        std::mem::forget(sock);
        
        Ok(())
    }
}

#[async_trait]
impl Connection for TcpConnection {
    async fn connect(&mut self) -> Result<()> {
        info!("Connecting to {}", self.address);
        
        let stream = TcpStream::connect(self.address).await
            .map_err(|e| BbcprError::Network(format!("Failed to connect: {}", e)))?;
        
        #[cfg(unix)]
        {
            Self::configure_socket(&stream)?;
        }
        
        self.stream = Some(stream);
        info!("TCP connection established");
        Ok(())
    }
    
    async fn send(&mut self, data: &[u8]) -> Result<usize> {
        let stream = self.stream.as_mut()
            .ok_or_else(|| BbcprError::Network("Not connected".to_string()))?;
        
        let bytes_written = stream.write(data).await
            .map_err(|e| BbcprError::Io(e))?;
        
        stream.flush().await
            .map_err(|e| BbcprError::Io(e))?;
        
        Ok(bytes_written)
    }
    
    async fn receive(&mut self, buf: &mut [u8]) -> Result<usize> {
        let stream = self.stream.as_mut()
            .ok_or_else(|| BbcprError::Network("Not connected".to_string()))?;
        
        let bytes_read = stream.read(buf).await
            .map_err(|e| BbcprError::Io(e))?;
        
        Ok(bytes_read)
    }
    
    async fn close(&mut self) -> Result<()> {
        if let Some(mut stream) = self.stream.take() {
            stream.shutdown().await
                .map_err(|e| BbcprError::Io(e))?;
            info!("TCP connection closed");
        }
        Ok(())
    }
}