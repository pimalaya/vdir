use io_fs::{coroutines::RemoveFile, Io};

use crate::Item;

#[derive(Debug)]
pub struct DeleteItem(RemoveFile);

impl DeleteItem {
    pub fn new(item: &Item) -> Self {
        Self(RemoveFile::new(item.path()))
    }

    pub fn resume(&mut self, io: Option<Io>) -> Result<(), Io> {
        self.0.resume(io)
    }
}
