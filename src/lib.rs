mod address_space;
mod cacher;
mod data_source;

use address_space::AddressSpace;
use data_source::FileDataSource;

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
        let data_source: FileDataSource = FileDataSource::new("Test data source");
        let offset: usize = 0;
        let length: usize = 1;

        data_source.add_map(addr_space, offset, length);

        assert_eq!(addr_space.mappings.is_empty(), false);
        assert_eq!(addr_space.mappings.front().source, Some(&data_source));
        assert_eq!(addr_space.mappings.front().offset, offset);
        assert_eq!(addr_space.mappings.front().span, length);
    }
}
