// bbcp protocol implementation

use crate::error::Result;
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, Clone)]
pub struct ProtocolMessage {
    pub message_type: MessageType,
    pub data: Bytes,
}

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Handshake = 0x01,
    FileInfo = 0x02,
    DataChunk = 0x03,
    Checksum = 0x04,
    Complete = 0x05,
    Error = 0x06,
}

impl ProtocolMessage {
    pub fn new(message_type: MessageType, data: Bytes) -> Self {
        Self { message_type, data }
    }
    
    pub fn encode(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(8 + self.data.len());
        buf.put_u32(self.message_type as u32);
        buf.put_u32(self.data.len() as u32);
        buf.put(self.data.clone());
        buf.freeze()
    }
    
    pub fn decode(mut data: Bytes) -> Result<Self> {
        if data.len() < 8 {
            return Err(crate::error::BbcprError::Protocol("Invalid message: too short".to_string()));
        }
        
        let msg_type = data.get_u32();
        let data_len = data.get_u32() as usize;
        
        if data.len() < data_len {
            return Err(crate::error::BbcprError::Protocol("Invalid message: data length mismatch".to_string()));
        }
        
        let message_type = match msg_type {
            0x01 => MessageType::Handshake,
            0x02 => MessageType::FileInfo,
            0x03 => MessageType::DataChunk,
            0x04 => MessageType::Checksum,
            0x05 => MessageType::Complete,
            0x06 => MessageType::Error,
            _ => return Err(crate::error::BbcprError::Protocol(format!("Unknown message type: {}", msg_type))),
        };
        
        let payload = data.split_to(data_len);
        
        Ok(Self {
            message_type,
            data: payload,
        })
    }
}