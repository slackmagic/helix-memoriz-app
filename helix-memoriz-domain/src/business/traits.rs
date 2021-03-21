use crate::business::error::EntryDomainResult;
use crate::core::{board::*, entry::*, label::*};

pub trait DomainTrait: Send {
    //ENTRY
    //-----------------------------------------------
    fn create_entry(&self, entry: Entry) -> EntryDomainResult<Option<Entry>>;
    fn update_entry(&self, entry: Entry) -> EntryDomainResult<Option<Entry>>;
    fn delete_entry(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid) -> EntryDomainResult<()>;
    fn archive_entry(&self, entry: Entry) -> EntryDomainResult<Option<Entry>>;
    fn undo_archive_entry(&self, entry: Entry) -> EntryDomainResult<Option<Entry>>;

    fn get_entry(
        &self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> EntryDomainResult<Option<Entry>>;

    fn get_all_entries(
        &self,
        owner_uuid: uuid::Uuid,
        archived_filter: Option<bool>,
    ) -> EntryDomainResult<Vec<Entry>>;
    fn get_all_entries_by_board(
        &self,
        owner_uuid: uuid::Uuid,
        board_uuid: uuid::Uuid,
        archived_filter: Option<bool>,
    ) -> EntryDomainResult<Vec<Entry>>;

    fn search(
        &self,
        owner_uuid: uuid::Uuid,
        content: Option<String>,
        board_uuid: Option<uuid::Uuid>,
        labels: Option<String>,
        archived_filter: Option<bool>,
    ) -> EntryDomainResult<Vec<Entry>>;

    // BOARD
    //-----------------------------------------------
    fn create_board(&self, board: Board) -> EntryDomainResult<Option<Board>>;
    fn update_board(&self, board: Board) -> EntryDomainResult<Option<Board>>;
    fn delete_board(&self, board: &Board) -> EntryDomainResult<()>;
    fn get_board(
        &self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> EntryDomainResult<Option<Board>>;
    fn get_all_boards(&self, owner_uuid: uuid::Uuid) -> EntryDomainResult<Vec<Board>>;

    /*
    // LABEL
    //-----------------------------------------------
    fn get_all_labels(&self) -> EntryDomainResult<Vec<Label>>;
    fn get_entry_labels(&self, id: i32) -> EntryDomainResult<Option<Vec<Label>>>;
    fn create_label<'a>(&self, label: Label) -> EntryDomainResult<Option<Label>>;
    fn update_label<'a>(&self, label: Label) -> EntryDomainResult<Option<Label>>;
    fn delete_label(&self, label: &Label) -> EntryDomainResult<()>;
    */
}
