use std::collections::LinkedList;
use std::sync::Arc;

use crate::data_source::DataSource;

type VirtualAddress = usize;

struct MapEntry {
    source: Arc<dyn DataSource>,
    offset: usize,
    span:   usize,
}
pub struct AddressSpace {
    name: String,
    mappings: LinkedList<MapEntry>,  // see below for comments
}

// comments about storing mappings
// Most OS code uses doubly-linked lists to store sparse data structures like
// an address space's mappings.
// Using Rust's built-in LinkedLists is fine. See https://doc.rust-lang.org/std/collections/struct.LinkedList.html
// But if you really want to get the zen of Rust, this is a really good read, written by the original author
// of that very data structure: https://rust-unofficial.github.io/too-many-lists/

// So, feel free to come up with a different structure, either a classic Rust collection,
// from a crate (but remember it needs to be #no_std compatible), or even write your own.

impl AddressSpace {
    pub fn new(name: &str) -> Self {
        AddressSpace {
            name: name.to_string(),
            mappings : LinkedList::new(),
        }
    }

    // add a mapping from DataSource into this AddressSpace
    // return VirtualAddress, or an error
    pub fn add_mapping(&self, source: &dyn DataSource, offset: usize, span: usize) -> Result<VirtualAddress, &str> {
        panic!("add mapping not yet implemented!");
    }

    // add a mapping from DataSource into this AddressSpace starting at start
    // returns Ok(), or an error if start + span doesn't have room for this mapping
    pub fn add_mapping_at(&self, source: &dyn DataSource, offset: usize, span: usize, start: VirtualAddress) -> Result<(), &str> {
        panic!("add mapping not yet implemented!");
    }

    // remove the mapping to DataSource that starts at VirtualAddress
    pub fn remove_mapping(&self, source: &dyn DataSource, start: VirtualAddress) -> Result<(), &str> {
        panic!("remove_mapping not yet implemented!");
    }
}