use std::path::PathBuf;

use io_fs::{coroutines::ReadFile, Io};

use crate::{item::ItemKind, Item};

#[derive(Debug)]
pub struct ReadItem {
    collection_path: PathBuf,
    item_name: String,
    item_kind: ItemKind,
    flow: ReadFile,
}

impl ReadItem {
    pub fn new(
        collection_path: impl Into<PathBuf>,
        item_name: impl ToString,
        item_kind: ItemKind,
    ) -> Self {
        let collection_path = collection_path.into();
        let item_name = item_name.to_string();
        let item_path = collection_path
            .join(&item_name)
            .with_extension(item_kind.as_extension());

        Self {
            collection_path,
            item_name,
            item_kind,
            flow: ReadFile::new(item_path),
        }
    }

    pub fn vcard(collection_path: impl Into<PathBuf>, item_name: impl ToString) -> Self {
        Self::new(collection_path, item_name, ItemKind::Vcard)
    }

    pub fn icalendar(collection_path: impl Into<PathBuf>, item_name: impl ToString) -> Self {
        Self::new(collection_path, item_name, ItemKind::Icalendar)
    }

    pub fn resume(&mut self, io: Option<Io>) -> Result<Item, Io> {
        let contents = self.flow.resume(io)?;

        let item = Item {
            collection_path: self.collection_path.clone(),
            kind: self.item_kind.clone(),
            name: self.item_name.clone(),
            contents,
        };

        Ok(item)
    }
}
