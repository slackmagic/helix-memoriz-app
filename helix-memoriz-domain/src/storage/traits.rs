use crate::core::{board::*, entry::*, label::*};
use crate::storage::error::*;

pub trait StorageTrait: Send {
    fn get_board(
        &mut self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> StorageResult<Option<Board>>;
    fn create_board(&mut self, board: Board) -> StorageResult<Board>;
    fn get_all_boards(&mut self, owner_uuid: uuid::Uuid) -> StorageResult<Vec<Board>>;
    fn update_board(&mut self, board: Board) -> StorageResult<Board>;
    fn delete_board(&mut self, board: &Board) -> StorageResult<()>;

    fn get_all_labels(&mut self) -> StorageResult<Vec<Label>>;
    fn create_label(&mut self, label: Label) -> StorageResult<Label>;
    fn update_label(&mut self, label: Label) -> StorageResult<Label>;
    fn delete_label(&mut self, label: &Label) -> StorageResult<()>;

    fn create_entry(&mut self, entry: Entry) -> StorageResult<Entry>;
    fn update_entry(&mut self, entry: Entry) -> StorageResult<Entry>;
    fn delete_entry(&mut self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid) -> StorageResult<()>;
    fn get_entry(
        &mut self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> StorageResult<Option<Entry>>;

    fn get_all_entries(&mut self, owner_uuid: uuid::Uuid) -> StorageResult<Vec<Entry>>;

    fn get_all_entries_by_board(
        &mut self,
        owner_uuid: uuid::Uuid,
        board_uuid: uuid::Uuid,
    ) -> StorageResult<Vec<Entry>>;
}
