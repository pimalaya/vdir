use std::path::PathBuf;

use fs_flows::{
    flows::{CreateFile, Rename},
    Io,
};

use crate::Item;

#[derive(Debug)]
pub enum State {
    CreateTemporaryItem(CreateFile),
    MoveItem(Rename),
}

#[derive(Debug)]
pub struct SaveItem {
    path: PathBuf,
    path_tmp: PathBuf,
    state: State,
}

impl SaveItem {
    pub fn new(item: Item) -> Self {
        let path = item.path.clone();
        let path_tmp = path.with_extension(format!("{}.tmp", item.extension()));
        let flow = CreateFile::new(&path_tmp, item.content);
        let state = State::CreateTemporaryItem(flow);

        Self {
            path,
            path_tmp,
            state,
        }
    }

    pub fn resume(&mut self, mut io: Option<Io>) -> Result<(), Io> {
        loop {
            match &mut self.state {
                State::CreateTemporaryItem(flow) => {
                    flow.resume(io.take())?;
                    let flow = Rename::new(&self.path_tmp, &self.path);
                    self.state = State::MoveItem(flow);
                }
                State::MoveItem(flow) => {
                    flow.resume(io.take())?;
                    break Ok(());
                }
            }
        }
    }
}
