#[path = "create-collection.rs"]
mod create_collection;
#[path = "create-item.rs"]
mod create_item;
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
#[path = "update-collection.rs"]
mod update_collection;
#[path = "update-item.rs"]
mod update_item;

#[doc(inline)]
pub use self::{
    create_collection::CreateCollection, create_item::CreateItem,
    delete_collection::DeleteCollection, delete_item::DeleteItem,
    list_collections::ListCollections, list_items::ListItems, read_item::ReadItem,
    update_collection::UpdateCollection, update_item::UpdateItem,
};
