use bincode::{config, Decode, Encode};
use crate::error::ServerError;

#[derive(Encode, Decode)]
pub struct Packet {
    pub magic: u16,
    pub length: usize,
    pub handler: u8,
    pub data: Vec<u8>
}

impl Packet {
    pub fn decode(slice: &[u8]) -> Result<Packet, ServerError> {
        let config = config::standard();
        bincode::decode_from_slice(slice, config)
            .map(|(decoded, _len)| decoded)
            .map_err(|e| e.into())
    }

    pub fn encode(&self) -> Result<Vec<u8>, ServerError> {
        let config = config::standard();
        bincode::encode_to_vec(self, config).map_err(|e| e.into())
    }
}