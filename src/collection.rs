use std::path::PathBuf;

/// The vdir collection.
///
/// A vdir collection is a directory that contains only files
/// (items). A collection may have [metadata], as defined in the
/// vdirsyncer standard.
///
/// See [`crate::Item`].
///
/// [metadata]: https://vdirsyncer.pimutils.org/en/stable/vdir.html#metadata
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Collection {
    /// The root path of the collection.
    ///
    /// Corresponds to the parent's directory path of the collection.
    pub root_path: PathBuf,

    /// The name of the collection.
    ///
    /// Corresponds to the name of the collection's directory.
    ///
    /// See also [`Self::display_name`].
    pub name: String,

    /// The display name of the collection.
    ///
    /// Files called displayname contain a UTF-8 encoded label, that
    /// may be used to represent the vdir in UIs.
    pub display_name: Option<String>,

    /// The description of the collection.
    ///
    /// Files called description contain a UTF-8 encoded description,
    /// that may be used to represent the vdir in UIs.
    pub description: Option<String>,

    /// The color of the collection.
    ///
    /// A file called color inside the vdir indicates the vdir’s
    /// color, a property that is only relevant in UI design.
    ///
    /// Its content is an ASCII-encoded hex-RGB value of the form
    /// #RRGGBB. For example, a file content of #FF0000 indicates that
    /// the vdir has a red (user-visible) color. No short forms or
    /// informal values such as red (as known from CSS, for example)
    /// are allowed. The prefixing # must be present.
    pub color: Option<String>,
}

impl Collection {
    pub fn path(&self) -> PathBuf {
        self.root_path.join(&self.name)
    }
}
