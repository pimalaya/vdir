use io_fs::{coroutines::CreateFile, Io};

use crate::{item, Item};

#[derive(Debug)]
pub struct CreateItem(CreateFile);

impl CreateItem {
    pub fn new(item: &Item) -> Self {
        let path = item::to_path_buf(item);
        Self(CreateFile::new(path, item.to_string().into_bytes()))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        self.0.resume(input)
    }
}
