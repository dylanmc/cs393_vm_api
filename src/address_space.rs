use std::collections::LinkedList;
use std::sync::Arc;

use crate::data_source::DataSource;

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
}