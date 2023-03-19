#![allow(dead_code, unused_variables)]
#![feature(linked_list_cursors)]

mod address_space;
mod cacher;
mod data_source;

pub use address_space::AddressSpace;
pub use data_source::{DataSource, FileDataSource};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors() {
        let _a = AddressSpace::new("my first address space");
        let _ds: FileDataSource = FileDataSource::new("Cargo.toml").unwrap(); // a little silly, but why not?
    }

    // more tests here - add mappings, read data, remove mappings and add more, make sure the
    // address space has what we expect in it after each operation

    // test if mapping has been added
    #[test]
    fn test_add_mapping() {
        let mut addr_space = AddressSpace::new("Test address space");
        let data_source: FileDataSource = FileDataSource::new("Cargo.toml").unwrap();
        let offset: usize = 0;
        let length: usize = 1;

        let addr = addr_space.add_mapping(data_source, offset, length).unwrap();
        assert!(addr != 0);

        // we should move these tests into addr_space, since they access non-public internals of the structure:
        // assert_eq!(addr_space.mappings.is_empty(), false);
        // assert_eq!(addr_space.mappings.front().source, Some(&data_source));
        // assert_eq!(addr_space.mappings.front().offset, offset);
        // assert_eq!(addr_space.mappings.front().span, length);
    }
}
