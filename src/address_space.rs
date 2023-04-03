//use std::collections::LinkedList;
use std::sync::Arc;

use crate::data_source::*;

pub const PAGE_SIZE: usize = 4096;
pub const VADDR_MAX: usize = (1 << 38) - 1;

type VirtualAddress = usize;

struct MapEntry {
    source: Arc<dyn DataSource>,
    offset: usize,
    span: usize,
    addr: usize,
    flags: FlagBuilder
}

impl MapEntry {
    #[must_use] // <- not using return value of "new" doesn't make sense, so warn
    pub fn new(source: Arc<dyn DataSource>, offset: usize, span: usize, addr: usize, flags: FlagBuilder) -> MapEntry {
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
        let mut ret = Self {
            name: name.to_string(),
            mappings: Vec::new(),
        };
        ret.add_mapping_at(
            Arc::new(NullDataSource {}),
            0,
            PAGE_SIZE,
            0,
            FlagBuilder::new() // all flags off by default
        ).unwrap();
        return ret;
    }

    /// Add a mapping from a `DataSource` into this `AddressSpace`.
    /// # Errors
    /// If the desired mapping is invalid.
    /// TODO: how does our test in lib.rs succeed?
    // pub fn add_mapping<'a, D: DataSource + 'a>( &'a mut self,
    pub fn add_mapping<D: DataSource + 'static>(
        &mut self,
        source: Arc<D>,
        offset: usize,
        span: usize,
        flags: FlagBuilder,
    ) -> Result<VirtualAddress, &str> {
        assert!(offset < VADDR_MAX);
        assert!(flags.is_valid());
        let span =
            if span % PAGE_SIZE == 0 {
                span
            } else {
                (span / PAGE_SIZE) * PAGE_SIZE + PAGE_SIZE
            }; // we only want to give out whole pages
        let mut i: usize = 0;
        while i < self.mappings.len()-1 {
            // note that there is already a null mapping preventing allocation of page 0
            if self.mappings[i].addr + self.mappings[i].span + span < self.mappings[i+1].addr {
                self.mappings.insert(
                    i,
                    MapEntry::new(
                        source,
                        offset,
                        span,
                        &self.mappings[i].addr + &self.mappings[i].span,
                        flags
                    ),
                );
                return Ok(offset);
            }
            i += 1;
        }
        if self.mappings.last().unwrap().addr + self.mappings.last().unwrap().span + span < VADDR_MAX {
            self.mappings.push(
                MapEntry::new(
                    source,
                    offset,
                    span,
                    &self.mappings.last().unwrap().addr + &self.mappings.last().unwrap().span,
                    flags
                )
            );
            return Ok(offset);
        }
        return Err("Unable to add virtual memory mapping.");
    }

    /// Add a mapping from `DataSource` into this `AddressSpace` starting at a specific address.
    /// # Errors
    /// If there is insufficient room subsequent to `start`.
    pub fn add_mapping_at<D: DataSource + 'static>(
        &mut self,
        source: Arc<D>,
        offset: usize,
        span: usize,
        start: VirtualAddress,
        flags: FlagBuilder
    ) -> Result<(), &str> {
        debug_assert!(start % PAGE_SIZE == 0);
        assert!(flags.is_valid());
        let span =
            if span % PAGE_SIZE == 0 {
                span
            } else {
                (span / PAGE_SIZE) * PAGE_SIZE + PAGE_SIZE
            }; // we only want to give out whole pages
        for m in &self.mappings {
            if m.addr < start && start < m.addr + m.span {
                return Err("Mapping already exists at addr.")
            }
            if m.addr < span && span < m.addr + m.span {
                return Err("Insufficient space subsequent to addr.")
            }
        }
        return Ok(());
    }

    /// Remove the mapping to `DataSource` that starts at the given address.
    /// # Errors
    /// If the mapping could not be removed.
    pub fn remove_mapping(
        &self,
        source: Arc<dyn DataSource>,
        start: VirtualAddress,
    ) -> Result<(), &str> {
        debug_assert!(start % PAGE_SIZE == 0);
        for m in &self.mappings {
            if m.addr == start && Arc::ptr_eq(&source, &m.source) {
                return Ok(());
            }
        }
        return Err("Could not remove mapping; no such mapping.");
    }

    /// Look up the DataSource and offset within that DataSource for a
    /// VirtualAddress / AccessType in this AddressSpace
    /// # Errors
    /// If this VirtualAddress does not have a valid mapping in &self,
    /// or if this AccessType is not permitted by the mapping
    pub fn get_source_for_addr(
        &self,
        addr: VirtualAddress,
        access_type: FlagBuilder
    ) -> Result<
            Option<
                (Arc<dyn DataSource>,
                 usize)
            >,
            &str
        > {
        for m in &self.mappings {
            if m.addr <= addr && addr < m.addr + m.span {
                if m.flags.check_access_perms(access_type) {
                    return Ok(Some((m.source.clone(), addr - m.addr)));
                    // I'm not quite sure whether to return addr - m.addr or m.offset here
                }
                else {
                    return Err("Insufficient permissions to access virtual address.");
                }
            }
        }
        return Ok(None);
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
    /// This is, somewhat counter-intuitively, named `and`, so that the following code reads
    /// correctly:
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
        Self { read, write, execute, cow, private, shared, }
    }

    pub const fn le(self, other: Self) -> bool {
        self.read <= other.read &&
        self.write <= other.write &&
        self.execute <= other.execute &&
        self.cow <= other.cow &&
        self.private <= other.private &&
        self.shared <= other.shared
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

    pub fn check_access_perms(&self, access_perms: FlagBuilder) -> bool {
        if access_perms.read && !self.read
        || access_perms.write && !self.write
        || access_perms.execute && !self.execute {
            false
        }
        true
    }

    pub fn is_valid(&self) -> bool {
        // for COW to work, write needs to be off until after the copy
        !(self.private && self.shared) && !(self.cow && self.write)
    }
}

