use serde::{Deserialize, Serialize};
use crate::error::ServerError;

pub trait Packet<'a> {
    type Target: Deserialize<'a>;

    fn decode(slice: &'a [u8]) -> Result<Self::Target, ServerError> {
        rmp_serde::decode::from_slice(slice).map_err(|e| e.into())
    }

    fn encode(&self) -> Result<Vec<u8>, ServerError>
        where Self: Serialize + Sized
    {
        rmp_serde::encode::to_vec_named(self).map_err(|e| e.into())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoutePacket {
    pub magic: u16,
    pub handler: u8,
}

impl<'a> Packet<'a> for RoutePacket {
    type Target = RoutePacket;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponsePacket {
    pub pack_id: u16,
    pub length: u32
}

impl<'a> Packet<'a> for ResponsePacket {
    type Target = ResponsePacket;
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorPacket {
    pub error: String
}

impl<'a> Packet<'a> for ErrorPacket {
    type Target = ErrorPacket;
}
