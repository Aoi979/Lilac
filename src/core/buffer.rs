use protobuf::{EnumOrUnknown, Message};

use kv::{KV, Types};
use crate::Error;
use crate::Error::{DataInvalid, KeyNotFound, TypeMissMatch};

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
#[derive(Debug, Clone)]
pub struct Buffer(KV);
pub struct  ReaderResult{
    pub data: Vec<u8>,
    pub types: Types,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Encoder: Send {
    fn encode_to_bytes(&self, raw_buffer: &Buffer, position: u32) -> Result<Vec<u8>>;
}

pub struct DecodeResult {
    pub buffer: Option<Buffer>,
    pub len: u32,
}

pub trait Decoder: Send {
    fn decode_bytes(&self, data: &[u8], position: u32) -> Result<DecodeResult>;
}

impl Buffer{
    fn from_kv(key: &str, t: Types, value: &[u8]) -> Self{
        let mut kv: KV = KV::new();
        kv.key = key.to_string();
        kv.type_ = EnumOrUnknown::new(t);
        kv.value = value.to_vec();
        Buffer(kv)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.write_to_bytes().unwrap()
    }

    pub fn key(&self) -> &str {
        self.0.key.as_str()
    }

    pub fn value(&self) -> &[u8] {
        self.0.value.as_slice()
    }

    pub fn is_deleting(&self) -> bool {
        if let Ok(buffer_type) = self.0.type_.enum_value() {
            buffer_type == Types::DELETED
        } else {
            false
        }
    }

    fn check_buffer_type(&self, required: Types) -> Result<()> {
        if self.is_deleting() {
            return Err(KeyNotFound);
        }
        if required == self.0.type_.enum_value().map_err(|_| TypeMissMatch)? {
            Ok(())
        } else {
            Err(TypeMissMatch)
        }
    }

    pub fn decode_str(&self) -> Result<String> {
        self.check_buffer_type(Types::STR)?;
        if let Ok(str) = String::from_utf8(self.0.value.to_vec()) {
            Ok(str)
        } else {
            Err(DataInvalid)
        }
    }

    pub fn decode_bool(&self) -> Result<bool> {
        self.check_buffer_type(Types::BYTE)?;
        Ok(self.0.value[0] == 1)
    }

    pub fn decode_byte_array(&self) -> Result<Vec<u8>> {
        self.check_buffer_type(Types::BYTE_ARRAY)?;
        Ok(self.0.value.to_vec())
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