use crate::core::buffer::{Buffer, Encoder};

pub struct BaseEncoder(Buffer);

impl Encoder for BaseEncoder{
    fn encode(&self, buffer: &Buffer, offset: u32) -> Result<Vec<u8>, Err()> {


    }
}