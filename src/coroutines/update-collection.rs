use std::{collections::HashMap, path::PathBuf};

use io_fs::{
    coroutines::{CreateFiles, Rename},
    Io,
};

use crate::{
    constants::{COLOR, DESCRIPTION, DISPLAYNAME, TMP},
    Collection,
};

#[derive(Debug)]
pub enum State {
    CreateMetadataFiles(CreateFiles, Vec<(PathBuf, PathBuf)>),
    MoveMetadataFiles(Rename),
}

#[derive(Debug)]
pub struct UpdateCollection {
    state: State,
}

impl UpdateCollection {
    pub fn new(mut collection: Collection) -> Self {
        let collection_path = collection.path();

        let mut contents = HashMap::new();
        let mut rename_paths = Vec::new();

        if let Some(name) = collection.display_name.take() {
            let path = collection_path.join(DISPLAYNAME);
            let tmp_path = path.with_extension(TMP);
            contents.insert(tmp_path.clone(), name.into_bytes());
            rename_paths.push((tmp_path, path));
        }

        if let Some(desc) = collection.description.take() {
            let path = collection_path.join(DESCRIPTION);
            let tmp_path = path.with_extension(TMP);
            contents.insert(tmp_path.clone(), desc.into_bytes());
            rename_paths.push((tmp_path, path));
        }

        if let Some(color) = collection.color.take() {
            let path = collection_path.join(COLOR);
            let tmp_path = path.with_extension(TMP);
            contents.insert(tmp_path.clone(), color.into_bytes());
            rename_paths.push((tmp_path, path));
        }

        let flow = CreateFiles::new(contents);
        let state = State::CreateMetadataFiles(flow, rename_paths);

        Self { state }
    }

    pub fn resume(&mut self, mut io: Option<Io>) -> Result<(), Io> {
        loop {
            match &mut self.state {
                State::CreateMetadataFiles(flow, rename_paths) => {
                    flow.resume(io.take())?;
                    let flow = Rename::new(rename_paths.drain(..));
                    self.state = State::MoveMetadataFiles(flow);
                }
                State::MoveMetadataFiles(flow) => {
                    flow.resume(io.take())?;
                    break Ok(());
                }
            }
        }
    }
}
