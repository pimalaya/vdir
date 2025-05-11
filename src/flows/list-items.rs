use std::{collections::HashSet, path::Path};

use fs_flows::{
    flows::{ReadDir, ReadFiles},
    Io,
};

use crate::{
    constants::{ICS, VCF},
    Item,
};

#[derive(Debug)]
pub enum State {
    ReadDir(ReadDir),
    ReadItems(ReadFiles),
}

#[derive(Debug)]
pub struct ListItems {
    state: State,
}

impl ListItems {
    pub fn new(collection_path: impl AsRef<Path>) -> Self {
        let flow = ReadDir::new(collection_path.as_ref());
        let state = State::ReadDir(flow);

        Self { state }
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
                        let Some(ext) = path.extension() else {
                            continue;
                        };

                        if ext == VCF {
                            items.insert(Item::vcard(path, contents));
                        } else if ext == ICS {
                            items.insert(Item::icalendar(path, contents));
                        }
                    }

                    break Ok(items);
                }
            }
        }
    }
}
