use std::path::PathBuf;

use io_fs::{
    coroutines::{CreateFile, Rename},
    Io,
};

use crate::Item;

#[derive(Debug)]
pub enum State {
    CreateTempItem(CreateFile),
    MoveItem(Rename),
}

#[derive(Debug)]
pub struct UpdateItem {
    path: PathBuf,
    path_tmp: PathBuf,
    state: State,
}

impl UpdateItem {
    pub fn new(item: &Item) -> Self {
        let path = item.path();
        let path_tmp = path.with_extension(format!("{}.tmp", item.extension()));
        let fs = CreateFile::new(&path_tmp, item.contents().into_bytes());
        let state = State::CreateTempItem(fs);

        Self {
            path,
            path_tmp,
            state,
        }
    }

    pub fn resume(&mut self, mut input: Option<Io>) -> Result<(), Io> {
        loop {
            match &mut self.state {
                State::CreateTempItem(fs) => {
                    fs.resume(input.take())?;
                    let fs = Rename::new(Some((&self.path_tmp, &self.path)));
                    self.state = State::MoveItem(fs);
                }
                State::MoveItem(fs) => {
                    fs.resume(input.take())?;
                    break Ok(());
                }
            }
        }
    }
}
