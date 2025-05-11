use std::path::PathBuf;

use crate::constants::{ICS, VCF};

/// The vdir collection's item.
///
/// A vdir collection's item is either a vCard (.vcf) or a iCalendar
/// file (.ics).
///
/// See [`crate::Collection`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Item {
    pub kind: ItemKind,
    pub path: PathBuf,
    pub content: Vec<u8>,
}

impl Item {
    pub fn vcard(path: impl Into<PathBuf>, content: impl IntoIterator<Item = u8>) -> Self {
        Self {
            kind: ItemKind::Vcard,
            path: path.into(),
            content: content.into_iter().collect(),
        }
    }

    pub fn icalendar(path: impl Into<PathBuf>, content: impl IntoIterator<Item = u8>) -> Self {
        Self {
            kind: ItemKind::Icalendar,
            path: path.into(),
            content: content.into_iter().collect(),
        }
    }

    pub fn extension(&self) -> &'static str {
        self.kind.extension()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ItemKind {
    Vcard,
    Icalendar,
}

impl ItemKind {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Vcard => VCF,
            Self::Icalendar => ICS,
        }
    }
}
