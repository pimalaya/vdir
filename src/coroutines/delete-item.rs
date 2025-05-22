use std::path::Path;

use io_fs::{coroutines::RemoveFile, Io};

#[derive(Debug)]
pub struct DeleteItem(RemoveFile);

impl DeleteItem {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(RemoveFile::new(path.as_ref()))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        self.0.resume(input)
    }
}
