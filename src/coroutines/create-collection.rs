use std::collections::HashMap;

use io_fs::{
    coroutines::{CreateDir, CreateFiles},
    Io,
};

use crate::{
    collection,
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
    pub fn new(collection: Collection) -> Self {
        let fs = CreateDir::new(collection::to_path_buf(&collection));
        let state = State::CreateCollection(fs);

        Self { collection, state }
    }

    pub fn resume(&mut self, mut input: Option<Io>) -> Result<(), Io> {
        loop {
            match &mut self.state {
                State::CreateCollection(fs) => {
                    fs.resume(input.take())?;

                    let display_name = self.collection.display_name.clone();
                    let description = self.collection.description.take();
                    let color = self.collection.color.take();
                    let collection_path = collection::to_path_buf(&self.collection);

                    let mut contents = HashMap::new();

                    if let Some(name) = display_name {
                        contents.insert(collection_path.join(DISPLAYNAME), name.into_bytes());
                    }

                    if let Some(desc) = description {
                        contents.insert(collection_path.join(DESCRIPTION), desc.into_bytes());
                    }

                    if let Some(color) = color {
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
