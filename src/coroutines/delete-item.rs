use io_fs::{coroutines::RemoveFile, Io};

use crate::{item, Item};

#[derive(Debug)]
pub struct DeleteItem(RemoveFile);

impl DeleteItem {
    pub fn new(item: &Item) -> Self {
        let path = item::to_path_buf(item);
        Self(RemoveFile::new(path))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        self.0.resume(input)
    }
}
