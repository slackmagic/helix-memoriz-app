use crate::business::error::EntryDomainResult;
use crate::core::{board::*, entry::*, label::*};

pub trait DomainTrait: Sync + Send {
    fn get_all_labels(&self) -> Vec<Label>;
    fn get_all_entries(&self, owner_uuid: uuid::Uuid, archived_filter: Option<bool>) -> Vec<Entry>;
    fn get_all_entries_by_board(
        &self,
        owner_uuid: uuid::Uuid,
        board_uuid: uuid::Uuid,
        archived_filter: Option<bool>,
    ) -> Vec<Entry>;
    fn get_board(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid) -> Option<Board>;
    fn get_all_boards(&self, owner_uuid: uuid::Uuid) -> Vec<Board>;
    fn get_entry(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid) -> Option<Entry>;
    fn get_entry_labels(&self, id: i32) -> Option<Vec<Label>>;

    fn search(
        &self,
        owner_uuid: uuid::Uuid,
        content: Option<String>,
        board_uuid: Option<uuid::Uuid>,
        labels: Option<String>,
        archived_filter: Option<bool>,
    ) -> Vec<Entry>;

    fn create_entry(&self, entry: Entry) -> Option<Entry>;
    fn update_entry(&self, entry: Entry) -> Option<Entry>;
    fn delete_entry(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid);
    fn archive_entry(&self, entry: Entry) -> Option<Entry>;
    fn undo_archive_entry(&self, entry: Entry) -> Option<Entry>;
    fn create_board(&self, board: Board) -> Option<Board>;
    fn update_board(&self, board: Board) -> Option<Board>;
    fn delete_board(&self, board: &Board);

    fn create_label<'a>(&self, label: Label) -> Option<Label>;
    fn update_label<'a>(&self, label: Label) -> Option<Label>;
    fn delete_label(&self, label: &Label);
}
