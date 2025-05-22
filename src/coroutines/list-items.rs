use std::{collections::HashSet, path::Path};

use calcard::{icalendar::ICalendar, vcard::VCard};
use io_fs::{
    coroutines::{ReadDir, ReadFiles},
    Io,
};

use crate::{
    constants::{ICS, VCF},
    item::ItemKind,
    Item,
};

#[derive(Debug)]
pub enum State {
    ListItems(ReadDir),
    ReadItems(ReadFiles),
}

#[derive(Debug)]
pub struct ListItems {
    state: State,
}

impl ListItems {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let fs = ReadDir::new(path.as_ref());
        let state = State::ListItems(fs);

        Self { state }
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
                                path,
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
                                path,
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
