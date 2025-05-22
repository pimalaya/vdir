use io_fs::{coroutines::CreateFile, Io};

use crate::Item;

#[derive(Debug)]
pub struct CreateItem(CreateFile);

impl CreateItem {
    pub fn new(item: Item) -> Self {
        let bytes = item.to_string().into_bytes();
        Self(CreateFile::new(item.path, bytes))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        self.0.resume(input)
    }
}
