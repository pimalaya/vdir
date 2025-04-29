use std::{collections::HashSet, io::ErrorKind};

use io_fs::runtimes::std::handle;
use io_vdir::{
    coroutines::{
        CreateCollection, CreateItem, DeleteCollection, DeleteItem, ListCollections, ListItems,
        ReadItem, UpdateCollection, UpdateItem,
    },
    Collection,
};
use tempdir::TempDir;

#[test]
fn std() {
    let root_path = TempDir::new("test-vdir-std-flows").unwrap().into_path();

    // should list empty collections

    let mut output = None;
    let mut flow = ListCollections::new(&root_path);

    let collections = loop {
        match flow.resume(output) {
            Ok(collections) => break collections,
            Err(input) => output = Some(handle(input).unwrap()),
        }
    };

    assert!(collections.is_empty());

    // should create collection without metadata

    let mut collection = Collection {
        root_path: root_path.clone(),
        name: "collection".into(),
        display_name: None,
        description: None,
        color: None,
    };

    let mut output = None;
    let mut flow = CreateCollection::new(collection.clone());

    while let Err(input) = flow.resume(output) {
        output = Some(handle(input).unwrap());
    }

    let mut output = None;
    let mut flow = ListCollections::new(&root_path);

    let collections = loop {
        match flow.resume(output) {
            Ok(collections) => break collections,
            Err(input) => output = Some(handle(input).unwrap()),
        }
    };

    let expected_collections = HashSet::from_iter([collection.clone()]);

    assert_eq!(collections, expected_collections);

    // should not re-create existing collection

    let mut output = None;
    let mut flow = CreateCollection::new(collection.clone());

    loop {
        match flow.resume(output) {
            Ok(()) => panic!("should not be OK"),
            Err(input) => match handle(input) {
                Ok(input) => output = Some(input),
                Err(err) => break assert_eq!(err.kind(), ErrorKind::AlreadyExists),
            },
        }
    }

    // should update collection with metadata

    collection.display_name = Some("Custom collection name".into());
    collection.description = Some("This is a description.".into());
    collection.color = Some("#000000".into());

    let mut output = None;
    let mut flow = UpdateCollection::new(collection.clone());

    while let Err(input) = flow.resume(output) {
        output = Some(handle(input).unwrap());
    }

    let mut output = None;
    let mut flow = ListCollections::new(&root_path);

    let items = loop {
        match flow.resume(output) {
            Ok(collections) => break collections,
            Err(input) => output = Some(handle(input).unwrap()),
        }
    };

    let expected_collections = HashSet::from_iter([collection.clone()]);

    assert_eq!(items, expected_collections);

    // should create item

    let mut output = None;
    let mut flow = CreateItem::vcard(collection.path(), "item", *b"UID: abc123");

    while let Err(input) = flow.resume(output) {
        output = Some(handle(input).unwrap());
    }

    let mut output = None;
    let mut flow = ListItems::new(collection.path());

    let items = loop {
        match flow.resume(output) {
            Ok(items) => break items,
            Err(input) => output = Some(handle(input).unwrap()),
        }
    };

    assert_eq!(items.len(), 1);

    let item = items.into_iter().next().unwrap();

    assert_eq!(item.name(), "item");
    assert_eq!(item.contents(), b"UID: abc123");

    // should update item

    let mut output = None;
    let mut flow = UpdateItem::new(&item, *b"UID: def456");

    while let Err(input) = flow.resume(output) {
        output = Some(handle(input).unwrap());
    }

    let mut output = None;
    let mut flow = ListItems::new(collection.path());

    let items = loop {
        match flow.resume(output) {
            Ok(items) => break items,
            Err(input) => output = Some(handle(input).unwrap()),
        }
    };

    assert_eq!(items.len(), 1);

    let item = items.into_iter().next().unwrap();

    assert_eq!(item.name(), "item");
    assert_eq!(item.contents(), b"UID: def456");

    // should read item

    let mut output = None;
    let mut flow = ReadItem::vcard(collection.path(), "item");

    let expected_item = loop {
        match flow.resume(output) {
            Ok(item) => break item,
            Err(input) => output = Some(handle(input).unwrap()),
        }
    };

    assert_eq!(item, expected_item);

    // should delete item

    let mut output = None;
    let mut flow = DeleteItem::new(&item);

    while let Err(input) = flow.resume(output) {
        output = Some(handle(input).unwrap());
    }

    let mut output = None;
    let mut flow = ListItems::new(collection.path());

    let items = loop {
        match flow.resume(output) {
            Ok(items) => break items,
            Err(input) => output = Some(handle(input).unwrap()),
        }
    };

    assert!(items.is_empty());

    // should delete collection

    let mut output = None;
    let mut flow = DeleteCollection::new(&collection);

    while let Err(input) = flow.resume(output) {
        output = Some(handle(input).unwrap());
    }

    let mut output = None;
    let mut flow = ListCollections::new(root_path);

    let collections = loop {
        match flow.resume(output) {
            Ok(collections) => break collections,
            Err(input) => output = Some(handle(input).unwrap()),
        }
    };

    assert!(collections.is_empty());
}
