use std::collections::{HashMap, HashSet};

use fs_flows::{
    flows::{CreateDir, CreateFiles, RemoveFiles},
    Io,
};

use crate::{
    constants::{COLOR, DESCRIPTION, DISPLAYNAME},
    Collection,
};

#[derive(Debug)]
pub enum State {
    CreateDir(CreateDir),
    RemoveMetadataFiles(RemoveFiles),
    CreateMetadataFiles(CreateFiles),
}

#[derive(Debug)]
pub struct SaveCollection {
    collection: Option<Collection>,
    state: State,
}

impl SaveCollection {
    pub fn new(collection: Collection) -> Self {
        let flow = CreateDir::new(&collection.path);
        let state = State::CreateDir(flow);

        Self {
            collection: Some(collection),
            state,
        }
    }

    pub fn resume(&mut self, mut io: Option<Io>) -> Result<(), Io> {
        loop {
            match &mut self.state {
                State::CreateDir(flow) => {
                    flow.resume(io.take())?;

                    let Some(collection) = &self.collection else {
                        return Err(Io::UnavailableInput);
                    };

                    let mut files = HashSet::new();

                    if collection.display_name.is_none() {
                        files.insert(collection.path.join(DISPLAYNAME));
                    }

                    if collection.description.is_none() {
                        files.insert(collection.path.join(DESCRIPTION));
                    }

                    if collection.color.is_none() {
                        files.insert(collection.path.join(COLOR));
                    }

                    let flow = RemoveFiles::new(files);
                    self.state = State::RemoveMetadataFiles(flow);
                }
                State::RemoveMetadataFiles(flow) => {
                    flow.resume(io.take())?;

                    let Some(mut collection) = self.collection.take() else {
                        return Err(Io::UnavailableInput);
                    };

                    let mut contents = HashMap::new();

                    if let Some(name) = collection.display_name.take() {
                        contents.insert(collection.path.join(DISPLAYNAME), name.into_bytes());
                    }

                    if let Some(desc) = collection.description.take() {
                        contents.insert(collection.path.join(DESCRIPTION), desc.into_bytes());
                    }

                    if let Some(color) = collection.color.take() {
                        contents.insert(collection.path.join(COLOR), color.into_bytes());
                    }

                    if contents.is_empty() {
                        break Ok(());
                    }

                    let flow = CreateFiles::new(contents.into_iter());
                    self.state = State::CreateMetadataFiles(flow);
                }
                State::CreateMetadataFiles(flow) => {
                    flow.resume(io.take())?;
                    break Ok(());
                }
            }
        }
    }
}
