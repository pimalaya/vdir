use std::collections::HashMap;

use io_fs::{
    coroutines::{CreateDir, CreateFiles},
    Io,
};

use crate::{
    constants::{COLOR, DESCRIPTION, DISPLAYNAME},
    Collection,
};

#[derive(Debug)]
pub enum State {
    CreateCollection(CreateDir),
    CreateMetadataFiles(CreateFiles),
}

#[derive(Debug)]
pub struct CreateCollection {
    collection: Collection,
    state: State,
}

impl CreateCollection {
    pub fn new(collection: &Collection) -> Self {
        let collection = collection.clone();
        let fs = CreateDir::new(collection.path());
        let state = State::CreateCollection(fs);

        Self { collection, state }
    }

    pub fn resume(&mut self, mut input: Option<Io>) -> Result<(), Io> {
        loop {
            match &mut self.state {
                State::CreateCollection(fs) => {
                    fs.resume(input.take())?;

                    let collection_path = self.collection.path();
                    let mut contents = HashMap::new();

                    if let Some(name) = self.collection.display_name.take() {
                        contents.insert(collection_path.join(DISPLAYNAME), name.into_bytes());
                    }

                    if let Some(desc) = self.collection.description.take() {
                        contents.insert(collection_path.join(DESCRIPTION), desc.into_bytes());
                    }

                    if let Some(color) = self.collection.color.take() {
                        contents.insert(collection_path.join(COLOR), color.into_bytes());
                    }

                    if contents.is_empty() {
                        break Ok(());
                    }

                    let fs = CreateFiles::new(contents);
                    self.state = State::CreateMetadataFiles(fs);
                }
                State::CreateMetadataFiles(fs) => {
                    fs.resume(input.take())?;
                    break Ok(());
                }
            }
        }
    }
}
