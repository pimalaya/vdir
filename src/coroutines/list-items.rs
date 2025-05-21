use std::collections::HashSet;

use calcard::{icalendar::ICalendar, vcard::VCard};
use io_fs::{
    coroutines::{ReadDir, ReadFiles},
    Io,
};

use crate::{
    collection,
    constants::{ICS, VCF},
    item::ItemKind,
    Collection, Item,
};

#[derive(Debug)]
pub enum State {
    ListItems(ReadDir),
    ReadItems(ReadFiles),
}

#[derive(Debug)]
pub struct ListItems {
    root: String,
    collection_id: String,
    state: State,
}

impl ListItems {
    pub fn new(collection: &Collection) -> Self {
        let root = collection.root().to_owned();
        let collection_id = collection.id().to_owned();
        let path = collection::to_path_buf(collection);
        let fs = ReadDir::new(path);
        let state = State::ListItems(fs);

        Self {
            root,
            collection_id,
            state,
        }
    }

    pub fn resume(&mut self, mut input: Option<Io>) -> Result<HashSet<Item>, Io> {
        loop {
            match &mut self.state {
                State::ListItems(fs) => {
                    let mut item_paths = fs.resume(input.take())?;

                    item_paths.retain(|path| {
                        if !path.is_file() {
                            return false;
                        }

                        let Some(ext) = path.extension() else {
                            return false;
                        };

                        if ext != VCF && ext != ICS {
                            return false;
                        }

                        return true;
                    });

                    let fs = ReadFiles::new(item_paths);
                    self.state = State::ReadItems(fs);
                }
                State::ReadItems(fs) => {
                    let contents = fs.resume(input.take())?;
                    let mut items = HashSet::new();

                    for (path, contents) in contents {
                        let Some(name) = path.file_stem() else {
                            continue;
                        };

                        let Some(ext) = path.extension() else {
                            continue;
                        };

                        let Ok(contents) = String::from_utf8(contents) else {
                            continue;
                        };

                        if ext == ICS {
                            let Ok(ical) = ICalendar::parse(contents) else {
                                continue;
                            };

                            if ical.uids().collect::<HashSet<_>>().len() != 1 {
                                continue;
                            }

                            items.insert(Item {
                                root: self.root.clone(),
                                collection_id: self.collection_id.clone(),
                                id: name.to_string_lossy().to_string(),
                                kind: ItemKind::Ical(ical),
                            });

                            continue;
                        }

                        if ext == VCF {
                            let Ok(vcard) = VCard::parse(contents) else {
                                continue;
                            };

                            if vcard.uid().is_none() {
                                continue;
                            }

                            items.insert(Item {
                                root: self.root.clone(),
                                collection_id: self.collection_id.clone(),
                                id: name.to_string_lossy().to_string(),
                                kind: ItemKind::Vcard(vcard),
                            });
                        }
                    }

                    break Ok(items);
                }
            }
        }
    }
}
