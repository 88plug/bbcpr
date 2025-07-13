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
    password: Option<String>,
    session: Option<Session>,
}

impl SshConnection {
    pub fn new(host: String, user: Option<String>, port: u16, identity_file: Option<String>) -> Self {
        Self {
            host,
            user,
            port,
            identity_file,
            password: None,
            session: None,
        }
    }

    pub fn with_password(mut self, password: Option<String>) -> Self {
        self.password = password;
        self
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
        
        // Try key-based authentication first if identity file is provided
        if let Some(ref identity) = self.identity_file {
            builder.keyfile(identity);
        }
        
        // If password is provided, configure password authentication
        if let Some(ref password) = self.password {
            // For openssh crate, password auth is handled via SSH_ASKPASS
            // We'll set an environment variable that the SSH client can use
            std::env::set_var("SSH_ASKPASS_REQUIRE", "force");
            std::env::set_var("DISPLAY", "none");
            
            // Create a temporary script that outputs the password
            let temp_dir = std::env::temp_dir();
            let askpass_script = temp_dir.join("bbcpr_askpass.sh");
            
            use std::fs::File;
            use std::io::Write;
            use std::os::unix::fs::PermissionsExt;
            
            let mut file = File::create(&askpass_script)
                .map_err(|e| BbcprError::Io(e))?;
            
            writeln!(file, "#!/bin/bash\necho '{}'", password)
                .map_err(|e| BbcprError::Io(e))?;
            
            // Make script executable
            let mut perms = file.metadata().map_err(|e| BbcprError::Io(e))?.permissions();
            perms.set_mode(0o755);
            file.set_permissions(perms).map_err(|e| BbcprError::Io(e))?;
            
            std::env::set_var("SSH_ASKPASS", askpass_script.to_string_lossy().to_string());
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