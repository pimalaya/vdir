use std::{collections::HashSet, path::PathBuf};

use calcard::{icalendar::ICalendar, vcard::VCard};
use io_fs::{
    coroutines::{ReadDir, ReadFiles},
    Io,
};

use crate::{
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
    collection_path: PathBuf,
    state: State,
}

impl ListItems {
    pub fn new(collection: &Collection) -> Self {
        let collection_path = collection.path();
        let fs = ReadDir::new(&collection_path);
        let state = State::ListItems(fs);

        Self {
            collection_path,
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
                                collection_path: self.collection_path.clone(),
                                kind: ItemKind::Ical(ical),
                                name: name.to_string_lossy().to_string(),
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
                                collection_path: self.collection_path.clone(),
                                kind: ItemKind::Vcard(vcard),
                                name: name.to_string_lossy().to_string(),
                            });
                        }
                    }

                    break Ok(items);
                }
            }
        }
    }
}
