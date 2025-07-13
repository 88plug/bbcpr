use async_trait::async_trait;
use openssh::{Session, SessionBuilder, Stdio};
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{debug, info};

use crate::error::{BbcprError, Result};
use crate::network::Connection;

pub struct SshConnection {
    host: String,
    user: Option<String>,
    port: u16,
    identity_file: Option<String>,
    session: Option<Session>,
}

impl SshConnection {
    pub fn new(host: String, user: Option<String>, port: u16, identity_file: Option<String>) -> Self {
        Self {
            host,
            user,
            port,
            identity_file,
            session: None,
        }
    }
}

#[async_trait]
impl Connection for SshConnection {
    async fn connect(&mut self) -> Result<()> {
        info!("Connecting to {}:{}", self.host, self.port);
        
        let mut builder = SessionBuilder::default();
        
        if let Some(ref user) = self.user {
            builder.user(user);
        }
        
        builder.port(self.port);
        
        if let Some(ref identity) = self.identity_file {
            builder.keyfile(identity);
        }
        
        let session = builder
            .connect(&self.host)
            .await
            .map_err(|e| BbcprError::Ssh(format!("Failed to connect: {}", e)))?;
        
        self.session = Some(session);
        info!("SSH connection established");
        Ok(())
    }
    
    async fn send(&mut self, data: &[u8]) -> Result<usize> {
        let session = self.session.as_ref()
            .ok_or_else(|| BbcprError::Network("Not connected".to_string()))?;
        
        // For bbcp protocol, we need to spawn the remote bbcp process
        let mut command = session
            .command("bbcp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .await
            .map_err(|e| BbcprError::Ssh(format!("Failed to spawn remote bbcp: {}", e)))?;
        
        let mut stdin = command.stdin().take()
            .ok_or_else(|| BbcprError::Ssh("Failed to get stdin".to_string()))?;
        
        let bytes_written = stdin.write(data).await
            .map_err(|e| BbcprError::Io(e))?;
        
        Ok(bytes_written)
    }
    
    async fn receive(&mut self, buf: &mut [u8]) -> Result<usize> {
        let session = self.session.as_ref()
            .ok_or_else(|| BbcprError::Network("Not connected".to_string()))?;
        
        // This is a simplified version - real implementation would maintain
        // the command handle and read from its stdout
        Err(BbcprError::Unsupported("SSH receive not yet implemented".to_string()))
    }
    
    async fn close(&mut self) -> Result<()> {
        if self.session.is_some() {
            self.session = None;
            info!("SSH connection closed");
        }
        Ok(())
    }
}