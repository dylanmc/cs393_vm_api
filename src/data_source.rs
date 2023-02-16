use crate::address_space::AddressSpace;
use std::fs::File;

pub trait DataSource {
    // constructors are left to each implementation, once you have one, you can:
    fn read(&self, offset: usize, length: usize, buffer: &mut Vec<u8> ) -> Result<(), &str>;
    fn write(&self, offset: usize, length: usize, buffer: &mut Vec<u8> ) -> Result<(), &str>;
    fn flush(&self, offset: usize, length: usize) -> Result<(), &str>;
    fn add_map(&self, into_address_space: &mut AddressSpace, offset: usize, length: usize) -> Result<usize, &str>;
    fn del_map(&self, from_address_space: &mut AddressSpace, offset: usize, length: usize) -> Result<(), &str>;
}

pub struct FileDataSource {
    file_handle: File,
    name: String,
}

impl FileDataSource {
    pub fn new(name: &str) -> Result<Self, &str> {
        if let Ok(f) = File::open(name){
            Ok(FileDataSource {
                file_handle: f,
                name: name.to_string(),
            })    
        } else {
            Err("couldn't open {name}")
        }
    }
}

impl DataSource for FileDataSource {
    fn read(&self, offset: usize, length: usize, buffer: &mut Vec<u8> ) -> Result<(), &str> {
        panic!("not yet done");
    }
    fn write(&self, offset: usize, length: usize, buffer: &mut Vec<u8> ) -> Result<(), &str>{
        panic!("not yet done");
    }
    fn flush(&self, offset: usize, length: usize) -> Result<(), &str>{
        panic!("not yet done");
    }
    fn add_map(&self, into_address_space: &mut AddressSpace, offset: usize, length: usize) -> Result<usize, &str>{
        panic!("not yet done");
    }
    fn del_map(&self, from_address_space: &mut AddressSpace, offset: usize, length: usize) -> Result<(), &str>{
        panic!("not yet done");
    }
}