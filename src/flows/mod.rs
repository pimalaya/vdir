#[path = "delete-collection.rs"]
mod delete_collection;
#[path = "delete-item.rs"]
mod delete_item;
#[path = "list-collections.rs"]
mod list_collections;
#[path = "list-items.rs"]
mod list_items;
#[path = "read-item.rs"]
mod read_item;
#[path = "save-collection.rs"]
mod save_collection;
#[path = "save-item.rs"]
mod save_item;

#[doc(inline)]
pub use self::{
    delete_collection::DeleteCollection, delete_item::DeleteItem,
    list_collections::ListCollections, list_items::ListItems, read_item::ReadItem,
    save_collection::SaveCollection, save_item::SaveItem,
};
