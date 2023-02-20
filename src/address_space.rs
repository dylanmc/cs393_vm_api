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
    mappings: LinkedList<MapEntry>,
}

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