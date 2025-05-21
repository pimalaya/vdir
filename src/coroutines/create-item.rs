use io_fs::{coroutines::CreateFile, Io};

use crate::Item;

#[derive(Debug)]
pub struct CreateItem(CreateFile);

impl CreateItem {
    pub fn new(item: &Item) -> Self {
        Self(CreateFile::new(item.path(), item.contents().into_bytes()))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        self.0.resume(input)
    }
}
