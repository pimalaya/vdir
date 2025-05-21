use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
};

use calcard::{icalendar::ICalendar, vcard::VCard};
use uuid::Uuid;

use crate::constants::{ICS, VCF};

/// The Vdir collection's item.
///
/// Represents either a vCard (.vcf) or a iCalendar file (.ics).
///
/// See [`crate::Collection`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    pub(crate) root: String,
    pub(crate) collection_id: String,
    pub(crate) id: String,

    /// The item kind.
    pub kind: ItemKind,
}

impl Item {
    pub fn new(root: impl ToString, collection_id: impl ToString, kind: ItemKind) -> Item {
        Item {
            root: root.to_string(),
            collection_id: collection_id.to_string(),
            id: Uuid::new_v4().to_string(),
            kind,
        }
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn collection_id(&self) -> &str {
        &self.collection_id
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.root.hash(state);
        self.collection_id.hash(state);
        self.id.hash(state);
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        match &self.kind {
            ItemKind::Ical(ical) => ical.to_string(),
            ItemKind::Vcard(vcard) => vcard.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ItemKind {
    Ical(ICalendar),
    Vcard(VCard),
}

impl ItemKind {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Ical(_) => ICS,
            Self::Vcard(_) => VCF,
        }
    }
}

pub fn to_path_buf(item: &Item) -> PathBuf {
    PathBuf::from(&item.root)
        .join(&item.collection_id)
        .join(&item.id)
        .with_extension(item.kind.extension())
}

pub fn to_path_buf_tmp(item: &Item) -> PathBuf {
    let ext = item.kind.extension();
    to_path_buf(item).with_extension(format!("{ext}.tmp"))
}
