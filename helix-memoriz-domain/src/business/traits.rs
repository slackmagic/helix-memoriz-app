use crate::business::error::EntryDomainResult;
use crate::core::{board::*, entry::*};
use async_trait::async_trait;

#[async_trait]
pub trait DomainTrait: Send + Sync {
    //ENTRY
    //-----------------------------------------------
    async fn create_entry(&self, entry: Entry) -> EntryDomainResult<Entry>;
    async fn update_entry(&self, entry: Entry) -> EntryDomainResult<Entry>;
    async fn delete_entry(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid)
        -> EntryDomainResult<()>;
    async fn archive_entry(
        &self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> EntryDomainResult<Entry>;
    async fn undo_archive_entry(
        &self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> EntryDomainResult<Entry>;

    async fn get_entry(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid)
        -> EntryDomainResult<Entry>;

    async fn get_all_entries(
        &self,
        owner_uuid: uuid::Uuid,
        archived_filter: Option<bool>,
    ) -> EntryDomainResult<Vec<Entry>>;
    async fn get_all_entries_by_board(
        &self,
        owner_uuid: uuid::Uuid,
        board_uuid: uuid::Uuid,
        archived_filter: Option<bool>,
    ) -> EntryDomainResult<Vec<Entry>>;

    async fn search(
        &self,
        owner_uuid: uuid::Uuid,
        content: Option<String>,
        board_uuid: Option<uuid::Uuid>,
        labels: Option<String>,
        archived_filter: Option<bool>,
    ) -> EntryDomainResult<Vec<Entry>>;

    // BOARD
    //-----------------------------------------------
    async fn create_board(&self, board: Board) -> EntryDomainResult<Board>;
    async fn update_board(&self, board: Board) -> EntryDomainResult<Board>;
    async fn delete_board(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid)
        -> EntryDomainResult<()>;
    async fn get_board(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid)
        -> EntryDomainResult<Board>;
    async fn get_all_boards(&self, owner_uuid: uuid::Uuid) -> EntryDomainResult<Vec<Board>>;

    /*
    // LABEL
    //-----------------------------------------------
    async fn get_all_labels(&self) -> EntryDomainResult<Vec<Label>>;
    async fn get_entry_labels(&self, id: i32) -> EntryDomainResult<Option<Vec<Label>>>;
    async fn create_label<'a>(&self, label: Label) -> EntryDomainResult<Option<Label>>;
    async fn update_label<'a>(&self, label: Label) -> EntryDomainResult<Option<Label>>;
    async fn delete_label(&self, label: &Label) -> EntryDomainResult<()>;
    */
}
