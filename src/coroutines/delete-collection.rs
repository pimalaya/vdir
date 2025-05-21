use io_fs::{coroutines::RemoveDir, Io};

use crate::{collection, Collection};

#[derive(Debug)]
pub struct DeleteCollection(RemoveDir);

impl DeleteCollection {
    pub fn new(collection: &Collection) -> Self {
        let path = collection::to_path_buf(collection);
        Self(RemoveDir::new(path))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        self.0.resume(input)
    }
}
