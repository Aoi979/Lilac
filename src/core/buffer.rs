use protobuf::EnumOrUnknown;

use kv::{KV, Types};

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
#[derive(Debug, Clone)]
pub struct Buffer(pub(crate) KV);
pub struct  ReaderResult{
    pub data: Vec<u8>,
    pub types: Types,
}

pub trait Encoder {
    fn encode(&self, buffer: &Buffer, offset: u32) -> Result<Vec<u8>, Err()>;
}
pub trait Decoder {
    fn reade(&self, data: Vec<u8>) -> Result<ReaderResult,Err()>;
}

impl Buffer{
    fn from_kv(key: &str, t: Types, value: &[u8]) -> Self{
        let mut kv: KV = KV::new();
        kv.key = key.to_string();
        kv.type_ = EnumOrUnknown::new(t);
        kv.value = value.to_vec();
        Buffer(kv)
    }

    pub fn from_byte_array(key: &str, value: &[u8]) -> Self {
        Buffer::from_kv(key, Types::BYTE_ARRAY, value)
    }

    pub fn from_bool(key: &str, value: bool) -> Self {
        let out = if value { 1u8 } else { 0u8 };
        Buffer::from_kv(key, Types::BYTE, vec![out].as_slice())
    }

    pub fn form_str(key: &str, value: &str) -> Self{
        Buffer::from_kv(key, Types::STR, value.as_bytes())
    }

    pub fn from_i32(key: &str, value: i32) -> Self{
        Buffer::from_kv(key,Types::I32,value.to_be_bytes().as_slice())
    }

    pub fn from_i64(key: &str, value: i64) -> Self{
        Buffer::from_kv(key,Types::I64,value.to_be_bytes().as_slice())
    }

    pub fn from_f32(key: &str, value: f32) -> Self{
        Buffer::from_kv(key,Types::F32,value.to_be_bytes().as_slice()())
    }

    pub fn from_f64(key: &str, value: f64) -> Self{
        Buffer::from_kv(key,Types::F64,value.to_be_bytes().as_slice())
    }

    pub fn form_i32_array(key: &str, value: &[i32]) -> Self{
        let mut vec = Vec::with_capacity(value.len() * 4);
        for i32 in  value {
            vec.extend_from_slice(i32.to_be_bytes().as_slice());
        }
        Buffer::from_kv(key,Types::I32_ARRAY,vec.as_slice())
    }

    pub fn form_i64_array(key: &str, value: &[i64]) -> Self{
        let mut vec = Vec::with_capacity(value.len() * 8);
        for i64 in  value {
            vec.extend_from_slice(i64.to_be_bytes().as_slice());
        }
        Buffer::from_kv(key,Types::I64_ARRAY,vec.as_slice())
    }

    pub fn form_f32_array(key: &str, value: &[f32]) -> Self{
        let mut vec = Vec::with_capacity(value.len() * 4);
        for f32 in  value{
            vec.extend_from_slice(f32.to_be_bytes().as_slice());
        }
        Buffer::from_kv(key,Types::F32_ARRAY,vec.as_slice())
    }
    pub fn form_f64_array(key: &str, value: &[f64]) -> Self{
        let mut vec = Vec::with_capacity(value.len() * 8);
        for f64 in  value{
            vec.extend_from_slice(f64.to_be_bytes().as_slice());
        }
        Buffer::from_kv(key,Types::F64_ARRAY,vec.as_slice())
    }


}