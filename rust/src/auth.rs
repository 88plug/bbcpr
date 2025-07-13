// Authentication utilities for bbcpr

use std::io::{self, Write};
use crate::error::{BbcprError, Result};

/// Prompt for password input (hidden from terminal)
pub fn prompt_password(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush().map_err(BbcprError::Io)?;
    
    // Try to use rpassword for hidden input
    #[cfg(feature = "rpassword")]
    {
        rpassword::read_password().map_err(|e| BbcprError::Io(e))
    }
    
    #[cfg(not(feature = "rpassword"))]
    {
        // Fallback to visible input with warning
        eprintln!("Warning: Password will be visible");
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(BbcprError::Io)?;
        Ok(input.trim().to_string())
    }
}

/// Get SSH password from various sources
pub fn get_ssh_password(
    password_prompt: bool,
    password_value: Option<String>,
    host: &str,
    user: Option<&str>
) -> Result<Option<String>> {
    if let Some(password) = password_value {
        // Password provided via command line (not recommended for security)
        Ok(Some(password))
    } else if password_prompt {
        // Interactive password prompt
        let user_part = user.map(|u| format!("{}@", u)).unwrap_or_default();
        let prompt = format!("Password for {}{}: ", user_part, host);
        let password = prompt_password(&prompt)?;
        
        if password.is_empty() {
            Ok(None)
        } else {
            Ok(Some(password))
        }
    } else {
        // No password authentication requested
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ssh_password_none() {
        let result = get_ssh_password(false, None, "example.com", Some("user"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_get_ssh_password_provided() {
        let result = get_ssh_password(false, Some("test123".to_string()), "example.com", Some("user"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("test123".to_string()));
    }
}