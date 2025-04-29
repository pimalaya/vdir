use std::{collections::HashSet, fs};

use fs_flows::{handlers::std::handle, Io};
use tempdir::TempDir;
use vdir::{flows::ListCollections, Collection};

#[test]
fn list_collections() {
    // set up vdir structure

    let tmp = TempDir::new("list-collections").unwrap();
    let root = tmp.path().join("root");
    let empty_collection = root.join("empty-collection");
    let full_collection = root.join("full-collection");

    fs::create_dir_all(&empty_collection).unwrap();
    fs::create_dir_all(&full_collection).unwrap();
    fs::write(full_collection.join("displayname"), b"full collection").unwrap();
    fs::write(full_collection.join("color"), b"#000000").unwrap();
    fs::write(
        full_collection.join("description"),
        b"this is a collection with full metadata",
    )
    .unwrap();

    // test starts here

    let mut flow = ListCollections::new(&root);
    let mut output = None;

    let collections = loop {
        match flow.resume(output) {
            Ok(collections) => break collections,
            Err(Io::Error(err)) => panic!("{err}"),
            Err(input) => output = Some(handle(input).unwrap()),
        }
    };

    let expected_collections = HashSet::from_iter([
        Collection {
            path: empty_collection.clone(),
            display_name: None,
            description: None,
            color: None,
        },
        Collection {
            path: full_collection.clone(),
            display_name: Some(String::from("full collection")),
            description: Some(String::from("this is a collection with full metadata")),
            color: Some(String::from("#000000")),
        },
    ]);

    assert_eq!(collections, expected_collections);
}
