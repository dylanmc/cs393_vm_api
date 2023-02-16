
mod address_space;
mod data_source;
mod mapper;

use address_space::AddressSpace;
use data_source::FileDataSource;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors() {
        let a = AddressSpace::new("my first address space");
        let ds: FileDataSource = FileDataSource::new("Cargo.toml").unwrap(); // a little silly, but why not?

        // loc = m.map(a, ds);
    }
}
