use std::{mem, path::PathBuf};

use calcard::{icalendar::ICalendar, vcard::VCard};
use io_fs::{coroutines::ReadFile, Io};

use crate::{
    constants::{ICS, VCF},
    Item, ItemKind,
};

#[derive(Debug)]
pub struct ReadItem {
    path: PathBuf,
    fs: ReadFile,
}

impl ReadItem {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let fs = ReadFile::new(&path);

        Self { path, fs }
    }

    pub fn resume(&mut self, input: Option<Io>) -> Result<Item, Io> {
        let p = self.path.display();

        let Some(ext) = self.path.extension() else {
            return Err(Io::error(format!("invalid item file extension at {p}")));
        };

        let contents = self.fs.resume(input)?;

        let Ok(contents) = String::from_utf8(contents) else {
            return Err(Io::error(format!("invalid item contents at {p}")));
        };

        if ext == VCF {
            let vcard = match VCard::parse(contents) {
                Ok(vcard) => vcard,
                Err(err) => {
                    let err = format!("invalid vcard contents at {p}: {err:?}");
                    return Err(Io::error(err));
                }
            };

            let item = Item {
                path: mem::take(&mut self.path),
                kind: ItemKind::Vcard(vcard),
            };

            return Ok(item);
        }

        if ext == ICS {
            let ical = match ICalendar::parse(contents) {
                Ok(ical) => ical,
                Err(err) => {
                    let err = format!("ical not readable at {p}: {err:?}");
                    return Err(Io::error(err));
                }
            };

            let item = Item {
                path: mem::take(&mut self.path),
                kind: ItemKind::Ical(ical),
            };

            return Ok(item);
        }

        let err = format!("invalid item file extension at {p}");
        Err(Io::error(err))
    }
}
