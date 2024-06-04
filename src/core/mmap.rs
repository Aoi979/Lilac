use std::{ptr, slice};
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::os::fd::RawFd;
use std::ptr::NonNull;

use libc::size_t;
use protobuf::plugin::code_generator_response::File;

const HEADER_OFFSET: usize = 8;
const MAP_POPULATE: libc::c_int = 0;
#[derive!(Debug)]
#[derive(Debug)]
struct MMap {
    pointer: NonNull<libc::c_void>,
    capacity: usize,
}

impl MMap{
    fn new(fd: RawFd, capacity: usize) -> Result<Self,Err()>{
        unsafe {
            let pointer = libc::mmap(
                ptr::null_mut(),
                capacity as size_t,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | MAP_POPULATE,
                fd,
                0,

            );
            if pointer == libc::MAP_FAILED {
                Err("MAP_FAILED")
            } else {
                libc::madvise(pointer,capacity,libc::MADV_WILLNEED);
                Ok(MMap{
                    pointer: NonNull::new(pointer).unwrap(),
                    capacity,
                })
            }
        }
    }

    fn flush(&self, capacity: usize) -> Result<Ok(),Err()> {
        let result = unsafe { libc::msync(self.pointer.as_ptr(), capacity as size_t, libc::MS_SYNC) };
        if result == 0 {
            Ok(())
        } else {
            Err("flush failed")
        }
    }
}

impl Drop for MMap {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.pointer.as_ptr(), self.capacity as size_t);
        }
    }
}

impl Deref for MMap {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.pointer.as_ptr() as * u8, self.capacity) }
    }
}

impl DerefMut for MMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            slice::from_raw_parts_mut(self.pointer.as_ptr() as * mut u8, self.capacity)
        }
    }
}


#[derive(Debug)]
pub struct MemoryMap (MMap);
impl MemoryMap{
    pub fn new(file: &File, capacity: usize) -> Self{
        let mmap = MMap::new(file.as_raw_fd(), capacity).unwrap();
        MemoryMap(mmap)
    }

    pub fn append(&self, value: Vec<u8>) -> Result<Ok,Err()>{
        let value_capacity = value.len();
        let start = usize::from_be_bytes(self.0[0..HEADER_OFFSET].try_into().unwrap()) + HEADER_OFFSET;
        let end = start+value_capacity;
        let total_capacity = start - HEADER_OFFSET + value_capacity;
        //更新头部的capacity
        self.0[0..HEADER_OFFSET].copy_from_slice(total_capacity.to_be_bytes().as_slice());
        self.0[start..end].copy_from_slice(value.as_slice());
        self.0.flush(end)
    }




}
