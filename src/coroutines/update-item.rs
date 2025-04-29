use std::path::PathBuf;

use io_fs::{
    coroutines::{CreateFile, Rename},
    Io,
};

use crate::Item;

#[derive(Debug)]
pub enum State {
    CreateTemporaryItem(CreateFile),
    MoveItem(Rename),
}

#[derive(Debug)]
pub struct UpdateItem {
    path: PathBuf,
    path_tmp: PathBuf,
    state: State,
}

impl UpdateItem {
    pub fn new(item: &Item, contents: impl IntoIterator<Item = u8>) -> Self {
        let path = item.path();
        let path_tmp = path.with_extension(format!("{}.tmp", item.extension()));
        let flow = CreateFile::new(&path_tmp, contents);
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
                    let flow = Rename::new(Some((&self.path_tmp, &self.path)));
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
