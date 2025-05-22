use std::{collections::HashSet, io::ErrorKind};

use calcard::vcard::VCard;
use io_fs::runtimes::std::handle;
use io_vdir::{coroutines::*, Collection, Item, ItemKind};
use tempdir::TempDir;

#[test]
fn std() {
    let workdir = TempDir::new("test-vdir-std").unwrap();
    let root = workdir.path();

    // should list empty collections

    let mut arg = None;
    let mut list = ListCollections::new(&root);

    let collections = loop {
        match list.resume(arg) {
            Ok(collections) => break collections,
            Err(io) => arg = Some(handle(io).unwrap()),
        }
    };

    assert!(collections.is_empty());

    // should create collection without metadata

    let mut collection = Collection::new(&root);

    let mut arg = None;
    let mut create = CreateCollection::new(collection.clone());

    while let Err(io) = create.resume(arg) {
        arg = Some(handle(io).unwrap());
    }

    let mut arg = None;
    let mut list = ListCollections::new(&root);

    let collections = loop {
        match list.resume(arg) {
            Ok(collections) => break collections,
            Err(io) => arg = Some(handle(io).unwrap()),
        }
    };

    let expected_collections = HashSet::from_iter([collection.clone()]);

    assert_eq!(collections, expected_collections);

    // should not re-create existing collection

    let mut arg = None;
    let mut create = CreateCollection::new(collection.clone());

    loop {
        match create.resume(arg) {
            Ok(()) => unreachable!("should not be OK"),
            Err(io) => match handle(io) {
                Ok(output) => arg = Some(output),
                Err(err) => break assert_eq!(err.kind(), ErrorKind::AlreadyExists),
            },
        }
    }

    // should update collection with metadata

    collection.display_name = Some("Custom collection name".into());
    collection.description = Some("This is a description.".into());
    collection.color = Some("#000000".into());

    let mut arg = None;
    let mut update = UpdateCollection::new(collection.clone());

    while let Err(io) = update.resume(arg) {
        arg = Some(handle(io).unwrap());
    }

    let mut arg = None;
    let mut list = ListCollections::new(&root);

    let items = loop {
        match list.resume(arg) {
            Ok(collections) => break collections,
            Err(io) => arg = Some(handle(io).unwrap()),
        }
    };

    let expected_collections = HashSet::from_iter([collection.clone()]);

    assert_eq!(items, expected_collections);

    // should create item

    let mut item = Item::new(
        &collection,
        ItemKind::Vcard(VCard::parse("BEGIN:VCARD\r\nUID: abc123\r\nEND:VCARD\r\n").unwrap()),
    );

    let mut arg = None;
    let mut create = CreateItem::new(item.clone());

    while let Err(io) = create.resume(arg) {
        arg = Some(handle(io).unwrap());
    }

    let mut arg = None;
    let mut list = ListItems::new(&collection);

    let items = loop {
        match list.resume(arg) {
            Ok(items) => break items,
            Err(io) => arg = Some(handle(io).unwrap()),
        }
    };

    assert_eq!(items.len(), 1);

    let first_item = items.into_iter().next().unwrap();

    assert_eq!(
        first_item.to_string(),
        "BEGIN:VCARD\r\nUID: abc123\r\nEND:VCARD\r\n"
    );

    // should update item

    item.kind =
        ItemKind::Vcard(VCard::parse("BEGIN:VCARD\r\nUID: def456\r\nEND:VCARD\r\n").unwrap());

    let mut arg = None;
    let mut update = UpdateItem::new(item);

    while let Err(io) = update.resume(arg) {
        arg = Some(handle(io).unwrap());
    }

    let mut arg = None;
    let mut list = ListItems::new(&collection);

    let items = loop {
        match list.resume(arg) {
            Ok(items) => break items,
            Err(io) => arg = Some(handle(io).unwrap()),
        }
    };

    assert_eq!(items.len(), 1);

    let item = items.into_iter().next().unwrap();

    assert_eq!(
        item.to_string(),
        "BEGIN:VCARD\r\nUID: def456\r\nEND:VCARD\r\n"
    );

    // // should read item

    // let mut output = None;
    // let mut fs = ReadItem::vcard(collection.path(), "item");

    // let expected_item = loop {
    //     match fs.resume(output) {
    //         Ok(item) => break item,
    //         Err(input) => output = Some(handle(input).unwrap()),
    //     }
    // };

    // assert_eq!(item, expected_item);

    // should delete item

    let mut arg = None;
    let mut delete = DeleteItem::new(item);

    while let Err(io) = delete.resume(arg) {
        arg = Some(handle(io).unwrap());
    }

    let mut arg = None;
    let mut list = ListItems::new(&collection);

    let items = loop {
        match list.resume(arg) {
            Ok(items) => break items,
            Err(io) => arg = Some(handle(io).unwrap()),
        }
    };

    assert_eq!(items.into_iter().count(), 0);

    // should delete collection

    let mut arg = None;
    let mut delete = DeleteCollection::new(&collection);

    while let Err(io) = delete.resume(arg) {
        arg = Some(handle(io).unwrap());
    }

    let mut arg = None;
    let mut list = ListCollections::new(root);

    let collections = loop {
        match list.resume(arg) {
            Ok(collections) => break collections,
            Err(io) => arg = Some(handle(io).unwrap()),
        }
    };

    assert!(collections.is_empty());

    workdir.close().unwrap();
}
