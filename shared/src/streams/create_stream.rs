use crate::bytes_serializable::BytesSerializable;
use crate::error::Error;
use std::str::from_utf8;

pub const MAX_NAME_LENGTH: usize = 100;

#[derive(Debug)]
pub struct CreateStream {
    pub stream_id: u32,
    pub name: String,
}

impl TryFrom<&[&str]> for CreateStream {
    type Error = Error;
    fn try_from(input: &[&str]) -> Result<Self, Self::Error> {
        if input.len() != 2 {
            return Err(Error::InvalidCommand);
        }

        let stream_id = input[0].parse::<u32>()?;
        let name = input[1].to_string();

        if name.len() > MAX_NAME_LENGTH {
            return Err(Error::InvalidStreamName);
        }

        Ok(CreateStream { stream_id, name })
    }
}

impl BytesSerializable for CreateStream {
    type Type = CreateStream;

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4 + self.name.len());
        bytes.extend_from_slice(&self.stream_id.to_le_bytes());
        bytes.extend_from_slice(self.name.as_bytes());
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self::Type, Error> {
        if bytes.len() < 5 {
            return Err(Error::InvalidCommand);
        }

        let stream_id = u32::from_le_bytes(bytes[..4].try_into()?);
        let name = from_utf8(&bytes[4..])?.to_string();
        if name.len() > MAX_NAME_LENGTH {
            return Err(Error::InvalidStreamName);
        }

        Ok(CreateStream { stream_id, name })
    }
}
