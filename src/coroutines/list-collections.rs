use std::{collections::HashSet, path::PathBuf};

use io_fs::{
    coroutines::{ReadDir, ReadFiles},
    Io,
};

use crate::{
    constants::{COLOR, DESCRIPTION, DISPLAYNAME},
    Collection,
};

#[derive(Debug)]
pub enum State {
    ListCollections(ReadDir),
    ReadMetadataFiles(HashSet<PathBuf>, ReadFiles),
}

#[derive(Debug)]
pub struct ListCollections {
    root_path: PathBuf,
    state: State,
}

impl ListCollections {
    pub fn new(root_path: impl Into<PathBuf>) -> Self {
        let root_path = root_path.into();
        let fs = ReadDir::new(&root_path);
        let state = State::ListCollections(fs);

        Self { root_path, state }
    }

    pub fn resume(&mut self, mut input: Option<Io>) -> Result<HashSet<Collection>, Io> {
        loop {
            match &mut self.state {
                State::ListCollections(fs) => {
                    let mut collection_paths = fs.resume(input.take())?;

                    collection_paths.retain(|path| {
                        let Some(name) = path.file_name() else {
                            return false;
                        };

                        let path = self.root_path.join(&name);

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
                }
                State::ReadMetadataFiles(collection_paths, fs) => {
                    let mut metadata = fs.resume(input.take())?;
                    let mut collections = HashSet::new();

                    for path in collection_paths.clone() {
                        let Some(name) = path.file_name() else {
                            continue;
                        };

                        let display_name = path.join(DISPLAYNAME);
                        let description = path.join(DESCRIPTION);
                        let color = path.join(COLOR);

                        let mut collection = Collection {
                            root_path: self.root_path.clone(),
                            name: name.to_string_lossy().to_string(),
                            display_name: None,
                            description: None,
                            color: None,
                        };

                        if let Some(name) = &metadata.remove(&display_name) {
                            let name = String::from_utf8_lossy(name);

                            if name.trim().is_empty() {
                                collection.display_name = None
                            } else {
                                collection.display_name = Some(name.to_string());
                            }
                        }

                        if let Some(desc) = &metadata.remove(&description) {
                            let desc = String::from_utf8_lossy(desc);

                            if desc.trim().is_empty() {
                                collection.description = None
                            } else {
                                collection.description = Some(desc.to_string());
                            }
                        }

                        if let Some(color) = &metadata.remove(&color) {
                            let color = String::from_utf8_lossy(color);

                            if color.trim().is_empty() {
                                collection.color = None
                            } else {
                                collection.color = Some(color.to_string());
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
