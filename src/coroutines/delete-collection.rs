use io_fs::{coroutines::RemoveDir, Io};

use crate::Collection;

#[derive(Debug)]
pub struct DeleteCollection(RemoveDir);

impl DeleteCollection {
    pub fn new(collection: &Collection) -> Self {
        Self(RemoveDir::new(collection.path()))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        self.0.resume(input)
    }
}
