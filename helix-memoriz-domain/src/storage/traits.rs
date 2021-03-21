use crate::core::{board::*, entry::*, label::*};
use crate::storage::error::*;

pub trait StorageTrait: Send {
    fn get_board(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid) -> StorageResult<Option<Board>>;
    fn create_board(&self, board: Board) -> StorageResult<Board>;
    fn get_all_boards(&self, owner_uuid: uuid::Uuid) -> StorageResult<Vec<Board>>;
    fn update_board(&self, board: Board) -> StorageResult<Board>;
    fn delete_board(&self, board: &Board) -> StorageResult<()>;

    fn get_all_labels(&self) -> StorageResult<Vec<Label>>;
    fn create_label(&self, label: Label) -> StorageResult<Label>;
    fn update_label(&self, label: Label) -> StorageResult<Label>;
    fn delete_label(&self, label: &Label) -> StorageResult<()>;

    fn create_entry(&self, entry: Entry) -> StorageResult<Entry>;
    fn update_entry(&self, entry: Entry) -> StorageResult<Entry>;
    fn delete_entry(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid) -> StorageResult<()>;
    fn get_entry(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid) -> StorageResult<Option<Entry>>;

    fn get_all_entries(&self, owner_uuid: uuid::Uuid) -> StorageResult<Vec<Entry>>;

    fn get_all_entries_by_board(
        &self,
        owner_uuid: uuid::Uuid,
        board_uuid: uuid::Uuid,
    ) -> StorageResult<Vec<Entry>>;
}
