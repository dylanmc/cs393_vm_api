#![allow(dead_code, unused_variables)]

mod address_space;
mod cacher;
mod data_source;

pub use address_space::AddressSpace;
pub use data_source::{DataSource, FileDataSource, FlagBuilder};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read;
    use std::ptr::null;

    #[test]
    fn constructors() {
        let _a = AddressSpace::new("my first address space");
        let _ds: FileDataSource = FileDataSource::new("Cargo.toml").unwrap(); // a little silly, but why not?
    }

    // more tests here - add mappings, read data, remove mappings and add more, make sure the
    // address space has what we expect in it after each operation

    #[test]
    fn test_del_map() {
        let mut addr_space = AddressSpace::new("mock address space");
        let data_source: Result<FileDataSource, &str> = FileDataSource::new("mock data source");

        let offset: usize = 0;
        let length: usize = 1;

        let data_source = data_source.unwrap();

        // add a mock mapping
        data_source
            .add_map(FlagBuilder::new(), &mut addr_space, offset, length)
            .expect("add mapping");

        // remove that mapping
        data_source
            .del_map(&mut addr_space, offset, length)
            .expect("remove mapping");

        // if mappings variable is empty, the removal was successful!
        assert!(addr_space.mappings_is_empty());
    }

    // test if mapping has been added
    #[test]
    fn test_add_mapping() {
        let mut addr_space = AddressSpace::new("Test address space");
        let data_source: FileDataSource = FileDataSource::new("Test data source");
        let offset: usize = 0;
        let length: usize = 1;

        addr_space.add_map(data_source, offset, length);

        assert_eq!(addr_space.mappings.is_empty(), false);
        assert_eq!(addr_space.mappings.front().source, Some(&data_source));
        assert_eq!(addr_space.mappings.front().offset, offset);
        assert_eq!(addr_space.mappings.front().span, length);
    }
}
