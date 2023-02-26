use std::sync::Arc;

use crate::data_source::DataSource;

pub const PAGE_SIZE: usize = 4096;
pub const VADDR_MAX: usize = (1 << 38) - 1;

type VirtualAddress = usize;

struct MapEntry {
    source: Arc<dyn DataSource>,
    offset: usize,
    span: usize,
    addr: usize,
}

impl MapEntry {
    #[must_use]
    pub fn new(source: Arc<dyn DataSource>, offset: usize, span: usize, addr: usize) -> MapEntry {
        MapEntry {
            source: source.clone(),
            offset,
            span,
            addr,
        }
    }
}

/// An address space.
pub struct AddressSpace {
    name: String,
    mappings: Vec<MapEntry>,
}

impl AddressSpace {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            mappings: Vec::new(),
        }
    }

    /// Add a mapping from a `DataSource` into this `AddressSpace`.
    ///
    /// # Errors
    /// If the desired mapping is invalid.
    pub fn add_mapping<D: DataSource + 'static>(
        &mut self,
        source: Arc<D>,
        offset: usize,
        span: usize,
    ) -> Result<VirtualAddress, &str> {
        let mut addr_iter = PAGE_SIZE;
        let mut gap;
        // find the free address by incrementing addr_iter
        for mapping in &self.mappings {
            gap = mapping.addr - addr_iter;
            if gap > span + 2 * PAGE_SIZE {
                break;
            }
            addr_iter = mapping.addr + mapping.span;
        }
        if addr_iter + span + 2 * PAGE_SIZE < VADDR_MAX {
            // compute the address for the new mapping
            let mapping_addr = addr_iter + PAGE_SIZE;
            let new_mapping = MapEntry::new(source, offset, span, mapping_addr);
            self.mappings.push(new_mapping);
            self.mappings.sort_by(|a, b| a.addr.cmp(&b.addr));
            return Ok(mapping_addr);
        }
        return Err("out of address space!");
    }

    /// Add a mapping from `DataSource` into this `AddressSpace` starting at a specific address.
    ///
    /// # Errors
    /// If there is insufficient room subsequent to `start`.
    pub fn add_mapping_at<D: DataSource + 'static>(
        &mut self,
        source: Arc<D>,
        offset: usize,
        span: usize,
        start: VirtualAddress,
    ) -> Result<(), &str> {
        if start + span + 2 * PAGE_SIZE < VADDR_MAX {
            let mapping_addr = start + PAGE_SIZE;
            let new_mapping = MapEntry::new(source, offset, span, mapping_addr);
            self.mappings.push(new_mapping);
            self.mappings.sort_by(|a, b| a.addr.cmp(&b.addr));
            return Ok(());
        }
        return Err("out of address space!");
    }

    /// Remove the mapping to `DataSource` that starts at the given address.
    ///
    /// # Errors
    /// If the mapping could not be removed.
    pub fn remove_mapping<D: DataSource + 'static>(
        &mut self,
        source: Arc<D>,
        start: VirtualAddress,
    ) -> Result<(), &str> {
        if start < VADDR_MAX {
            if let Ok(mapping_index) = self.get_mapping_index_for_addr(start) {
                self.mappings.remove(mapping_index);
            }
        }
        return Err("cannot remove the mapping!");
    }

    /// Look up the DataSource and offset within that DataSource for a
    /// VirtualAddress / AccessType in this AddressSpace
    ///
    /// # Errors
    /// If this VirtualAddress does not have a valid mapping in &self,
    /// or if this AccessType is not permitted by the mapping
    pub fn get_source_for_addr(
        &self,
        addr: VirtualAddress,
        access_type: FlagBuilder,
    ) -> Result<Arc<(dyn DataSource + 'static)>, &str> {
        if addr < VADDR_MAX {
            self.mappings
                .iter()
                .find(|m| m.addr == addr)
                .map(|m| m.source.clone());
        }
        return Err("address is out of bounds!");
    }

    /// Helper function for looking up mapping index
    fn get_mapping_index_for_addr(&self, addr: VirtualAddress) -> Result<usize, &str> {
        if addr < VADDR_MAX {
            self.mappings.iter().position(|m| m.addr == addr);
        }
        return Err("address is out of bounds!");
    }
}

/// Build flags for address space maps.
///
/// We recommend using this builder type as follows:
/// ```
/// # use reedos_address_space::FlagBuilder;
/// let flags = FlagBuilder::new()
///     .toggle_read()
///     .toggle_write();
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)] // clippy is wrong: bools are more readable than enums
                                         // here because these directly correspond to yes/no
                                         // hardware flags
pub struct FlagBuilder {
    // TODO: should there be some sanity checks that conflicting flags are never toggled? can we do
    // this at compile-time? (the second question is maybe hard)
    read: bool,
    write: bool,
    execute: bool,
    cow: bool,
    private: bool,
    shared: bool,
}

/// Create a constructor and toggler for a `FlagBuilder` object. Will capture attributes, including documentation
/// comments and apply them to the generated constructor.
macro_rules! flag {
    (
        $flag:ident,
        $toggle:ident
    ) => {
        #[doc=concat!("Turn on only the ", stringify!($flag), " flag.")]
        #[must_use]
        pub fn $flag() -> Self {
            Self {
                $flag: true,
                ..Self::default()
            }
        }

        #[doc=concat!("Toggle the ", stringify!($flag), " flag.")]
        #[must_use]
        pub const fn $toggle(self) -> Self {
            Self {
                $flag: !self.$flag,
                ..self
            }
        }
    };
}

impl FlagBuilder {
    /// Create a new `FlagBuilder` with all flags toggled off.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    flag!(read, toggle_read);
    flag!(write, toggle_write);
    flag!(execute, toggle_execute);
    flag!(cow, toggle_cow);
    flag!(private, toggle_private);
    flag!(shared, toggle_shared);

    #[must_use]
    /// Combine two `FlagBuilder`s by boolean or-ing each of their flags.
    ///
    /// This is, somewhat counter-intuitively, named `and`, so that the following code reads
    /// correctly:
    ///
    /// ```
    /// # use reedos_address_space::FlagBuilder;
    /// let read = FlagBuilder::read();
    /// let execute = FlagBuilder::execute();
    /// let new = read.and(execute);
    /// assert_eq!(new, FlagBuilder::new().toggle_read().toggle_execute());
    /// ```
    pub const fn and(self, other: Self) -> Self {
        let read = self.read || other.read;
        let write = self.write || other.write;
        let execute = self.execute || other.execute;
        let cow = self.cow || other.cow;
        let private = self.private || other.private;
        let shared = self.shared || other.shared;

        Self {
            read,
            write,
            execute,
            cow,
            private,
            shared,
        }
    }

    #[must_use]
    /// Turn off all flags in self that are on in other.
    ///
    /// You can think of this as `self &! other` on each field.
    ///
    /// ```
    /// # use reedos_address_space::FlagBuilder;
    /// let read_execute = FlagBuilder::read().toggle_execute();
    /// let execute = FlagBuilder::execute();
    /// let new = read_execute.but_not(execute);
    /// assert_eq!(new, FlagBuilder::new().toggle_read());
    /// ```
    pub const fn but_not(self, other: Self) -> Self {
        let read = self.read && !other.read;
        let write = self.write && !other.write;
        let execute = self.execute && !other.execute;
        let cow = self.cow && !other.cow;
        let private = self.private && !other.private;
        let shared = self.shared && !other.shared;

        Self {
            read,
            write,
            execute,
            cow,
            private,
            shared,
        }
    }
}
