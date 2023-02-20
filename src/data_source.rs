use crate::address_space::AddressSpace;
use std::fs::File;

pub trait DataSource {
    // constructors are left to each implementation, once you have one, you can:
    //
    // TODO: instead of taking a `flagbuilder`, should we turn it into some kind of convenient
    // format?
    fn read(&self, offset: usize, length: usize, buffer: &mut Vec<u8>) -> Result<(), &str>;
    fn write(&self, offset: usize, length: usize, buffer: &mut Vec<u8>) -> Result<(), &str>;
    fn flush(&self, offset: usize, length: usize) -> Result<(), &str>;
    fn add_map(
        &self,
        with_flag: FlagBuilder,
        into_address_space: &mut AddressSpace,
        offset: usize,
        length: usize,
    ) -> Result<usize, &str>;
    fn del_map(
        &self,
        from_address_space: &mut AddressSpace,
        offset: usize,
        length: usize,
    ) -> Result<(), &str>;
}

/// Build flags for data source maps.
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

#[allow(clippy::module_name_repetitions)]
pub struct FileDataSource {
    file_handle: File,
    name: String,
}

impl FileDataSource {
    /// Create a new `FileDataSource`.
    ///
    /// # Errors
    /// If the file can't be opened.
    pub fn new(name: &str) -> Result<Self, &str> {
        File::open(name).map_or(Err("couldn't open {name}"), |file_handle| {
            Ok(Self {
                file_handle,
                name: name.to_string(),
            })
        })
    }
}

impl DataSource for FileDataSource {
    fn read(&self, offset: usize, length: usize, buffer: &mut Vec<u8>) -> Result<(), &str> {
        todo!()
    }
    fn write(&self, offset: usize, length: usize, buffer: &mut Vec<u8>) -> Result<(), &str> {
        todo!()
    }
    fn flush(&self, offset: usize, length: usize) -> Result<(), &str> {
        todo!()
    }
    fn add_map(
        &self,
        with_flag: FlagBuilder,
        into_address_space: &mut AddressSpace,
        offset: usize,
        length: usize,
    ) -> Result<usize, &str> {
        todo!()
    }
    fn del_map(
        &self,
        from_address_space: &mut AddressSpace,
        offset: usize,
        length: usize,
    ) -> Result<(), &str> {
        todo!()
    }
}
