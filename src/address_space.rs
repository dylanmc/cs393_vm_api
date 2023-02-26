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
    flags: FlagBuilder,
}

impl MapEntry {
    #[must_use] // <- not using return value of "new" doesn't make sense, so warn
    pub fn new(
        source: Arc<dyn DataSource>,
        offset: usize,
        span: usize,
        addr: usize,
        flags: FlagBuilder,
    ) -> MapEntry {
        MapEntry {
            source: source.clone(),
            offset,
            span,
            addr,
            flags,
        }
    }
}

/// An address space.
pub struct AddressSpace {
    name: String,
    mappings: Vec<MapEntry>, // see below for comments
}

// comments about storing mappings
// Most OS code uses doubly-linked lists to store sparse data structures like
// an address space's mappings.
// Using Rust's built-in LinkedLists is fine. See https://doc.rust-lang.org/std/collections/struct.LinkedList.html
// But if you really want to get the zen of Rust, this is a really good read, written by the original author
// of that very data structure: https://rust-unofficial.github.io/too-many-lists/

// So, feel free to come up with a different structure, either a classic Rust collection,
// from a crate (but remember it needs to be #no_std compatible), or even write your own.
// See this ticket from Riley: https://github.com/dylanmc/cs393_vm_api/issues/10

impl AddressSpace {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            mappings: Vec::new(), // <- here I changed from LinkedList, for reasons
        } // I encourage you to try other sparse representations - trees, DIY linked lists, ...
    }

    /// Add a mapping from a `DataSource` into this `AddressSpace`.
    ///
    /// # Errors
    /// If the desired mapping is invalid.
    /// TODO: how does our test in lib.rs succeed?
    // pub fn add_mapping<'a, D: DataSource + 'a>(
    //     &'a mut self,
    pub fn add_mapping<D: DataSource + 'static>(
        &mut self,
        source: Arc<D>,
        offset: usize,
        span: usize,
        flags: FlagBuilder,
    ) -> Result<VirtualAddress, &str> {
        let mut addr_iter = PAGE_SIZE; // let's not map page 0
        let mut gap;
        let adjusted_span: usize;
        if span % PAGE_SIZE != 0 {
            adjusted_span = PAGE_SIZE - (PAGE_SIZE % span);
        } else {
            adjusted_span = span
        }
        for mapping in &self.mappings {
            // space between end of last entry and start of this entry
            gap = mapping.addr - addr_iter;
            // we found a big free space!
            // need to have a free page between each mapping
            // for accidental overflows
            if gap > adjusted_span + 2 * PAGE_SIZE {
                break;
            }
            // move addr_iter to the next potentially free chunk of memory
            addr_iter = mapping.addr + mapping.span;
        }
        // if we have enough space for our new mapping
        if addr_iter + adjusted_span + 2 * PAGE_SIZE < VADDR_MAX {
            // add free page between mappings
            let mapping_addr = addr_iter + PAGE_SIZE;
            let new_mapping = MapEntry::new(source, offset, adjusted_span, mapping_addr, flags);
            self.mappings.push(new_mapping);
            self.mappings.sort_by(|a, b| a.addr.cmp(&b.addr));
            return Ok(mapping_addr);
        }
        Err("out of address space!")
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
        flags: FlagBuilder,
    ) -> Result<(), &str> {
        todo!()
    }

    /// Remove the mapping to `DataSource` that starts at the given address.
    ///
    /// # Errors
    /// If the mapping could not be removed.
    pub fn remove_mapping<D: DataSource>(
        &self,
        source: Arc<D>,
        start: VirtualAddress,
    ) -> Result<(), &str> {
        todo!()
    }

    /// Look up the DataSource and offset within that DataSource for a
    /// VirtualAddress / AccessType in this AddressSpace
    ///
    /// # Errors
    /// If this VirtualAddress does not have a valid mapping in &self,
    /// or if this AccessType is not permitted by the mapping
    pub fn get_source_for_addr<D: DataSource>(
        &self,
        addr: VirtualAddress,
        access_type: FlagBuilder,
    ) -> Result<(Arc<D>, usize), &str> {
        todo!();
    }

    /// Helper function for looking up mappings
    fn get_mapping_for_addr(&self, addr: VirtualAddress) -> Result<MapEntry, &str> {
        todo!();
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

impl FlagBuilder {
    pub fn check_access_perms(&self, access_perms: FlagBuilder) -> bool {
        if access_perms.read && !self.read
            || access_perms.write && !self.write
            || access_perms.execute && !self.execute
        {
            return false;
        }
        true
    }

    pub fn is_valid(&self) -> bool {
        if self.private && self.shared {
            return false;
        }
        if self.cow && self.write {
            // for COW to work, write needs to be off until after the copy
            return false;
        }
        return true;
    }
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
