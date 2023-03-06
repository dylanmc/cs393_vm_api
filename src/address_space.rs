use std::collections::LinkedList;
use std::sync::Arc;

use crate::data_source::DataSource;

const SPACE: usize = 2usize.pow(27) - 1;

const PAGE: usize = 4096;

type VirtualAddress = usize;

#[derive(Clone)]
struct DataView {
    source: Arc<dyn DataSource>,
    offset: usize,
}

#[derive(Clone)]
struct MapEntry {
    content: Option<DataView>,
    span: usize,
    addr: usize,
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
// See this ticket from Riley: https://github.com/dylanmc/cs393_vm_api/issues/10

impl AddressSpace {
    #[must_use]
    pub fn new(name: &str) -> Self {
        let mut s = Self {
            name: name.to_string(),
            mappings: LinkedList::new(),
        };
        s.mappings.push_back(MapEntry {
            content: None,
            span: (SPACE / PAGE - 2) * PAGE,
            addr: PAGE,
        });
        s
    }

    /// Add a mapping from a `DataSource` into this `AddressSpace`.
    ///
    /// # Errors
    /// If the desired mapping is invalid.
    pub fn add_mapping<D: DataSource + 'static>(
        &mut self,
        source: D,
        offset: usize,
        span: usize,
    ) -> Result<VirtualAddress, &str> {
        let fullspan = PAGE * ((span - 1) / PAGE + 1);
        let mut c = self.mappings.cursor_front_mut();
        loop {
            let node = c.current().cloned();
            match node {
                Some(entry) => {
                    if entry.content.is_none() && entry.span >= fullspan {
                        let leftover = entry.span - fullspan;
                        c.insert_before(MapEntry {
                            content: Some(DataView {
                                source: Arc::new(source),
                                offset,
                            }),
                            span: fullspan,
                            addr: entry.addr,
                        });
                        if leftover >= PAGE {
                            c.insert_before(MapEntry { 
                                content: None, 
                                span: leftover - PAGE, 
                                addr: entry.addr + fullspan + PAGE, 
                            })
                        }
                        c.remove_current();
                        return Ok(entry.addr);
                    } else {
                        c.move_next();
                    }
                }
                None => return Err("no available mappings of that size."),
            }
        }
    }

    /// Add a mapping from `DataSource` into this `AddressSpace` starting at a specific address.
    ///
    /// # Errors
    /// If there is insufficient room subsequent to `start`.
    pub fn add_mapping_at<D: DataSource + 'static>(
        &mut self,
        source: D,
        offset: usize,
        span: usize,
        start: VirtualAddress,
    ) -> Result<(), &str> {
        let fullspan = PAGE * ((span - 1) / PAGE + 1);
        let mut c = self.mappings.cursor_front_mut();
        loop {
            let node = c.current().cloned();
            match node {
                Some(entry) => {
                    if entry.content.is_none() && start >= entry.addr && entry.addr + entry.span - start >= fullspan {
                        let before = start - entry.span;
                        let leftover = entry.addr + entry.span - start - fullspan;
                        if before >= PAGE {
                            c.insert_before(MapEntry { 
                                content: None, 
                                span: before - PAGE, 
                                addr: entry.addr, 
                            })
                        }
                        c.insert_before(MapEntry {
                            content: Some(DataView {
                                source: Arc::new(source),
                                offset,
                            }),
                            span: fullspan,
                            addr: start,
                        });
                        if leftover >= PAGE {
                            c.insert_before(MapEntry { 
                                content: None, 
                                span: leftover - PAGE, 
                                addr: start + fullspan + PAGE, 
                            })
                        }
                        c.remove_current();
                        return Ok(());
                    } else {
                        c.move_next();
                    }
                }
                None => return Err("no available mapping at that address."),
            }
        }
    }

    /// Remove the mapping to `DataSource` that starts at the given address.
    ///
    /// # Errors
    /// If the mapping could not be removed.
    pub fn remove_mapping<D: DataSource>(
        &self,
        source: &D,
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
    ) -> Result<(&D, usize), &str> {
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
