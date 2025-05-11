use fs_flows::{flows::ReadFile, Io};

use crate::Item;

#[derive(Debug)]
pub struct ReadItem(ReadFile);

impl ReadItem {
    pub fn new(item: &Item) -> Self {
        Self(ReadFile::new(&item.path))
    }

    pub fn resume(&mut self, io: Option<Io>) -> Result<Vec<u8>, Io> {
        self.0.resume(io)
    }
}
