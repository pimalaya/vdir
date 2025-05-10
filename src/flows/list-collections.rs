use std::{collections::HashSet, path::PathBuf};

use fs_flows::{
    flows::{ReadDir, ReadFiles},
    Io,
};

use crate::{
    constants::{COLOR, DESCRIPTION, DISPLAYNAME},
    Collection,
};

#[derive(Debug)]
pub enum State {
    ReadDirs(ReadDir),
    ReadMetadataFiles(HashSet<PathBuf>, ReadFiles),
}

#[derive(Debug)]
pub struct ListCollections {
    root: PathBuf,
    state: State,
}

impl ListCollections {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        let flow = ReadDir::new(&root);
        let state = State::ReadDirs(flow);

        Self { root, state }
    }

    pub fn resume(&mut self, mut io: Option<Io>) -> Result<HashSet<Collection>, Io> {
        loop {
            match &mut self.state {
                State::ReadDirs(flow) => {
                    let mut collection_paths = flow.resume(io.take())?;

                    collection_paths.retain(|path| {
                        let Some(name) = path.file_name() else {
                            return false;
                        };

                        let path = self.root.join(&name);

                        if !path.is_dir() {
                            return false;
                        }

                        true
                    });

                    let mut metadata_paths = HashSet::new();

                    for dir in &collection_paths {
                        let name_path = dir.join(DISPLAYNAME);

                        if name_path.is_file() {
                            metadata_paths.insert(name_path);
                        }

                        let desc_path = dir.join(DESCRIPTION);

                        if desc_path.is_file() {
                            metadata_paths.insert(desc_path);
                        }

                        let color_path = dir.join(COLOR);

                        if color_path.is_file() {
                            metadata_paths.insert(color_path);
                        }
                    }

                    let flow = ReadFiles::new(metadata_paths);
                    self.state = State::ReadMetadataFiles(collection_paths, flow);

                    continue;
                }
                State::ReadMetadataFiles(collection_paths, flow) => {
                    let mut metadata = flow.resume(io.take())?;
                    let mut collections = HashSet::new();

                    for path in collection_paths.clone() {
                        let display_name = path.join(DISPLAYNAME);
                        let description = path.join(DESCRIPTION);
                        let color = path.join(COLOR);

                        let mut collection = Collection {
                            path,
                            display_name: None,
                            description: None,
                            color: None,
                        };

                        if let Some(ref name) = metadata.remove(&display_name) {
                            let name = String::from_utf8_lossy(name);

                            if name.trim().is_empty() {
                                collection.display_name = None
                            } else {
                                collection.display_name.replace(name.to_string());
                            }
                        }

                        if let Some(ref desc) = metadata.remove(&description) {
                            let desc = String::from_utf8_lossy(desc);

                            if desc.trim().is_empty() {
                                collection.description = None
                            } else {
                                collection.description.replace(desc.to_string());
                            }
                        }

                        if let Some(ref color) = metadata.remove(&color) {
                            let color = String::from_utf8_lossy(color);

                            if color.trim().is_empty() {
                                collection.color = None
                            } else {
                                collection.color.replace(color.to_string());
                            }
                        }

                        collections.insert(collection);
                    }

                    break Ok(collections);
                }
            }
        }
    }
}
