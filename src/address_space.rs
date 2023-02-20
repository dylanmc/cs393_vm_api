use std::collections::LinkedList;
use std::sync::Arc;

use crate::data_source::DataSource;

type VirtualAddress = usize;

struct MapEntry {
    source: Arc<dyn DataSource>,
    offset: usize,
    span: usize,
}

/// An address space.
pub struct AddressSpace {
    name: String,
    mappings: LinkedList<MapEntry>, // see below for comments
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
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            mappings: LinkedList::new(),
        }
    }

    /// Add a mapping from a `DataSource` into this `AddressSpace`.
    ///
    /// # Errors
    /// If the desired mapping is invalid.
    pub fn add_mapping(
        &self,
        source: &dyn DataSource,
        offset: usize,
        span: usize,
    ) -> Result<VirtualAddress, &str> {
        todo!()
    }

    /// Add a mapping from `DataSource` into this `AddressSpace` starting at a specific address.
    ///
    /// # Errors
    /// If there is insufficient room subsequent to `start`.
    pub fn add_mapping_at(
        &self,
        source: &dyn DataSource,
        offset: usize,
        span: usize,
        start: VirtualAddress,
    ) -> Result<(), &str> {
        todo!()
    }

    /// Remove the mapping to `DataSource` that starts at the given address.
    /// # Errors
    /// If the mapping could not be removed.
    pub fn remove_mapping(
        &self,
        source: &dyn DataSource,
        start: VirtualAddress,
    ) -> Result<(), &str> {
        todo!()
    }
}
