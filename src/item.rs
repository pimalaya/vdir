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
    pub collection_path: PathBuf,
    pub kind: ItemKind,
    pub name: String,
    pub contents: Vec<u8>,
}

impl Item {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn contents(&self) -> &[u8] {
        &self.contents
    }

    pub fn path(&self) -> PathBuf {
        self.collection_path
            .join(&self.name)
            .with_extension(self.kind.as_extension())
    }

    pub fn extension(&self) -> &'static str {
        self.kind.as_extension()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ItemKind {
    Vcard,
    Icalendar,
}

impl ItemKind {
    pub fn as_extension(&self) -> &'static str {
        match self {
            Self::Vcard => VCF,
            Self::Icalendar => ICS,
        }
    }
}
