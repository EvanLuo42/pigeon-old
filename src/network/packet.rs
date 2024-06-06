use bincode::{config, Decode, Encode};
use crate::error::ServerError;

#[derive(Encode, Decode)]
pub struct RequestPacket {
    pub magic: u16,
    pub length: usize,
    pub handler: u8,
    pub data: Vec<u8>
}

impl RequestPacket {
    pub fn decode(slice: &[u8]) -> Result<RequestPacket, ServerError> {
        let config = config::standard();
        bincode::decode_from_slice(slice, config)
            .map(|(decoded, _len)| decoded)
            .map_err(|e| e.into())
    }
}