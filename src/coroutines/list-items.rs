use std::{collections::HashSet, path::PathBuf};

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
    ReadDir(ReadDir),
    ReadItems(ReadFiles),
}

#[derive(Debug)]
pub struct ListItems {
    collection_path: PathBuf,
    state: State,
}

impl ListItems {
    pub fn new(collection_path: impl Into<PathBuf>) -> Self {
        let collection_path = collection_path.into();
        let flow = ReadDir::new(&collection_path);
        let state = State::ReadDir(flow);

        Self {
            collection_path,
            state,
        }
    }

    pub fn resume(&mut self, mut io: Option<Io>) -> Result<HashSet<Item>, Io> {
        loop {
            match &mut self.state {
                State::ReadDir(flow) => {
                    let mut item_paths = flow.resume(io.take())?;

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

                    let flow = ReadFiles::new(item_paths);
                    self.state = State::ReadItems(flow);

                    continue;
                }
                State::ReadItems(flow) => {
                    let contents = flow.resume(io.take())?;
                    let mut items = HashSet::new();

                    for (path, contents) in contents {
                        let Some(name) = path.file_stem() else {
                            continue;
                        };

                        let Some(ext) = path.extension() else {
                            continue;
                        };

                        if ext == VCF {
                            items.insert(Item {
                                collection_path: self.collection_path.clone(),
                                kind: ItemKind::Vcard,
                                name: name.to_string_lossy().to_string(),
                                contents,
                            });
                        } else if ext == ICS {
                            items.insert(Item {
                                collection_path: self.collection_path.clone(),
                                kind: ItemKind::Icalendar,
                                name: name.to_string_lossy().to_string(),
                                contents,
                            });
                        }
                    }

                    break Ok(items);
                }
            }
        }
    }
}
