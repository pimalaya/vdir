use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
};

use calcard::{icalendar::ICalendar, vcard::VCard};

use crate::constants::{ICS, VCF};

/// The Vdir collection's item.
///
/// Represents either a vCard (.vcf) or a iCalendar file (.ics).
///
/// See [`crate::Collection`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    pub collection_path: PathBuf,
    pub name: String,
    pub kind: ItemKind,
}

impl Item {
    pub fn path(&self) -> PathBuf {
        self.collection_path
            .join(&self.name)
            .with_extension(self.extension())
    }

    pub fn extension(&self) -> &'static str {
        self.kind.extension()
    }

    pub fn contents(&self) -> String {
        match &self.kind {
            ItemKind::Ical(ical) => ical.to_string(),
            ItemKind::Vcard(vcard) => vcard.to_string(),
        }
    }
}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path().hash(state)
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
