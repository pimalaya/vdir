use std::path::Path;

use io_fs::{coroutines::CreateFile, Io};

use crate::ItemKind;

#[derive(Debug)]
pub struct CreateItem {
    flow: CreateFile,
}

impl CreateItem {
    pub fn new(
        collection_path: impl AsRef<Path>,
        kind: ItemKind,
        name: impl AsRef<str>,
        contents: impl IntoIterator<Item = u8>,
    ) -> Self {
        let path = collection_path
            .as_ref()
            .join(name.as_ref())
            .with_extension(kind.as_extension());

        Self {
            flow: CreateFile::new(path, contents),
        }
    }

    pub fn vcard(
        collection_path: impl AsRef<Path>,
        name: impl AsRef<str>,
        contents: impl IntoIterator<Item = u8>,
    ) -> Self {
        Self::new(collection_path, ItemKind::Vcard, name, contents)
    }

    pub fn icalendar(
        collection_path: impl AsRef<Path>,
        name: impl AsRef<str>,
        contents: impl IntoIterator<Item = u8>,
    ) -> Self {
        Self::new(collection_path, ItemKind::Icalendar, name, contents)
    }

    pub fn resume(&mut self, io: Option<Io>) -> Result<(), Io> {
        self.flow.resume(io)
    }
}
