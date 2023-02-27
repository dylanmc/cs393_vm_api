#![allow(dead_code, unused_variables)]

mod address_space;
mod cacher;
mod data_source;

pub use address_space::{AddressSpace, FlagBuilder};
pub use data_source::{DataSource, FileDataSource};
use std::sync::Arc; // <- will have to make Arc ourselves for #no_std
                    // TODO: why does rustfmt say this is unused, but if I leave it out, undefined?

#[cfg(test)]
mod tests {
    use crate::address_space::PAGE_SIZE;

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
        let read_flags = FlagBuilder::new().toggle_read();

        let ds_arc = Arc::new(data_source);

        let addr = addr_space
            .add_mapping(ds_arc.clone(), offset, length, read_flags)
            .unwrap();
        assert!(addr != 0);

        let addr2 = addr_space
            .add_mapping(ds_arc.clone(), address_space::PAGE_SIZE, 0, read_flags)
            .unwrap();
        assert!(addr2 != 0);
        assert!(addr2 != addr);
    }

    // test running out of address space
    #[test]
    #[should_panic]
    fn test_add_mapping_error() {
        let mut addr_space = AddressSpace::new("Test address space for errors");
        let data_source: FileDataSource = FileDataSource::new("Cargo.toml").unwrap();
        let offset: usize = 0;
        let length: usize = (1 << 38);
        let read_flags = FlagBuilder::new().toggle_read();

        let ds_arc = Arc::new(data_source);

        let addr = addr_space
            .add_mapping(ds_arc.clone(), offset, length, read_flags)
            .unwrap();
    }

    // test if mapping has been added
    #[test]
    fn test_add_mapping_at() {
        let mut addr_space = AddressSpace::new("Test address space 1");
        let data_source: FileDataSource = FileDataSource::new("Cargo.toml").unwrap();
        let offset: usize = 0;
        let length: usize = 1;
        let start: usize = 100;
        let read_flags = FlagBuilder::new().toggle_read();

        let ds_arc = Arc::new(data_source);

        let addr = addr_space
            .add_mapping_at(ds_arc.clone(), offset, length, start, read_flags)
            .unwrap();
    }

    // test adding multiple files to the same VA
    #[test]
    #[should_panic]
    fn test_add_mapping_at_error() {
        let mut addr_space = AddressSpace::new("Test address space for errors 1");
        let data_source: FileDataSource = FileDataSource::new("Cargo.toml").unwrap();
        let offset: usize = 0;
        let length: usize = 1;
        let start: usize = 2 * PAGE_SIZE;
        let read_flags = FlagBuilder::new().toggle_read();

        let ds_arc = Arc::new(data_source);

        let addr = addr_space
            .add_mapping_at(ds_arc.clone(), offset, length, start, read_flags)
            .unwrap();
        let addr = addr_space
            .add_mapping_at(ds_arc.clone(), offset, length, start, read_flags)
            .unwrap();
    }

    // test adding a file without read permissions
    #[test]
    #[should_panic]
    fn test_add_mapping_at_error_access() {
        let mut addr_space = AddressSpace::new("Test address space for errors 2");
        let data_source: FileDataSource = FileDataSource::new("Cargo.toml").unwrap();
        let offset: usize = 0;
        let length: usize = 1;
        let start: usize = 2 * PAGE_SIZE;

        let ds_arc = Arc::new(data_source);

        let addr = addr_space
            .add_mapping_at(ds_arc.clone(), offset, length, start, read_flags)
            .unwrap();
    }
}
