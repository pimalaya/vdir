use std::path::PathBuf;

/// The vdir collection's item.
///
/// A vdir collection's item is either a vCard (.vcf) or a iCalendar
/// file (.ics).
///
/// See [`crate::Collection`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Item {
    kind: ItemKind,
    path: PathBuf,
    content: Vec<u8>,
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
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ItemKind {
    Vcard,
    Icalendar,
}
