use std::fs::File;

pub trait DataSource {
    // constructors are left to each implementation, once you have one, you can:
    //
    // TODO: instead of taking a `flagbuilder`, should we turn it into some kind of convenient
    // format?
    //
    // TODO: add documentation for all these methods
    fn read(&self, offset: usize, length: usize, buffer: &mut [u8]) -> Result<(), &str>;
    fn write(&self, offset: usize, length: usize, buffer: &[u8]) -> Result<(), &str>;
    fn flush(&self, offset: usize, length: usize) -> Result<(), &str>;
}

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
    fn read(&self, offset: usize, length: usize, buffer: &mut [u8]) -> Result<(), &str> {
        todo!()
    }
    fn write(&self, offset: usize, length: usize, buffer: &[u8]) -> Result<(), &str> {
        todo!()
    }
    fn flush(&self, offset: usize, length: usize) -> Result<(), &str> {
        todo!()
    }
}
