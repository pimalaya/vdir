use std::path::Path;

use io_fs::{coroutines::RemoveDir, Io};

#[derive(Debug)]
pub struct DeleteCollection(RemoveDir);

impl DeleteCollection {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(RemoveDir::new(path.as_ref()))
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<(), Io> {
        self.0.resume(input)
    }
}
