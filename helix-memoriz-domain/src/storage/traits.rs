use crate::core::{board::*, entry::*, label::*};
use crate::storage::error::*;
use async_trait::async_trait;

#[async_trait]
pub trait StorageTrait: Send + Sync {
    async fn get_board(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid) -> StorageResult<Board>;
    async fn create_board(&self, board: Board) -> StorageResult<Board>;
    async fn get_all_boards(&self, owner_uuid: uuid::Uuid) -> StorageResult<Vec<Board>>;
    async fn update_board(&self, board: Board) -> StorageResult<Board>;
    async fn delete_board(&self, board: &Board) -> StorageResult<()>;

    async fn get_all_labels(&self) -> StorageResult<Vec<Label>>;
    async fn create_label(&self, label: Label) -> StorageResult<Label>;
    async fn update_label(&self, label: Label) -> StorageResult<Label>;
    async fn delete_label(&self, label: &Label) -> StorageResult<()>;

    async fn create_entry(&self, entry: Entry) -> StorageResult<Entry>;
    async fn update_entry(&self, entry: Entry) -> StorageResult<Entry>;
    async fn delete_entry(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid) -> StorageResult<()>;
    async fn get_entry(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid) -> StorageResult<Entry>;

    async fn get_all_entries(&self, owner_uuid: uuid::Uuid) -> StorageResult<Vec<Entry>>;

    async fn get_all_entries_by_board(
        &self,
        owner_uuid: uuid::Uuid,
        board_uuid: uuid::Uuid,
    ) -> StorageResult<Vec<Entry>>;
}
