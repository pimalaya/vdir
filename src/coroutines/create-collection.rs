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
    CreateDir(CreateDir),
    CreateMetadataFiles(CreateFiles),
}

#[derive(Debug)]
pub struct CreateCollection {
    collection: Option<Collection>,
    state: State,
}

impl CreateCollection {
    pub fn new(collection: Collection) -> Self {
        let flow = CreateDir::new(collection.path());
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

                    let Some(mut collection) = self.collection.take() else {
                        return Err(Io::UnavailableInput);
                    };

                    let collection_path = collection.path();
                    let mut contents = HashMap::new();

                    if let Some(name) = collection.display_name.take() {
                        contents.insert(collection_path.join(DISPLAYNAME), name.into_bytes());
                    }

                    if let Some(desc) = collection.description.take() {
                        contents.insert(collection_path.join(DESCRIPTION), desc.into_bytes());
                    }

                    if let Some(color) = collection.color.take() {
                        contents.insert(collection_path.join(COLOR), color.into_bytes());
                    }

                    if contents.is_empty() {
                        break Ok(());
                    }

                    let flow = CreateFiles::new(contents);
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
