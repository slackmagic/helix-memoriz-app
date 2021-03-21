use crate::business::error::EntryDomainResult;
use crate::business::error::MemorizDomainError;
use crate::business::traits::DomainTrait;
use crate::core::board::Board;
use crate::core::entry::Entry;
use crate::core::label::Label;
use crate::storage::traits::StorageTrait;
use std::boxed::Box;

pub struct MemorizDomain {
    storage: Box<dyn StorageTrait>,
}

impl MemorizDomain {
    pub fn new(storage: Box<dyn StorageTrait>) -> Self {
        MemorizDomain { storage }
    }
}

impl DomainTrait for MemorizDomain {
    fn get_all_entries(
        &self,
        owner_uuid: uuid::Uuid,
        archived_filter: Option<bool>,
    ) -> EntryDomainResult<Vec<Entry>> {
        //TODO: Filter on archived
        Ok(self.storage.get_all_entries(owner_uuid)?)
    }

    fn get_all_entries_by_board(
        &self,
        owner_uuid: uuid::Uuid,
        board_uuid: uuid::Uuid,
        archived_filter: Option<bool>,
    ) -> EntryDomainResult<Vec<Entry>> {
        Ok(self
            .storage
            .get_all_entries_by_board(owner_uuid, board_uuid)?)
    }

    fn get_board(
        &self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> EntryDomainResult<Option<Board>> {
        Ok(self.storage.get_board(owner_uuid, uuid)?)
    }

    fn get_all_boards(&self, owner_uuid: uuid::Uuid) -> EntryDomainResult<Vec<Board>> {
        Ok(self.get_all_boards(owner_uuid)?)
    }

    fn get_entry(
        &self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> EntryDomainResult<Option<Entry>> {
        Ok(self.storage.get_entry(owner_uuid, uuid)?)
    }

    fn search(
        &self,
        owner_uuid: uuid::Uuid,
        content: Option<String>,
        board_uuid: Option<uuid::Uuid>,
        labels: Option<String>,
        archived_filter: Option<bool>,
    ) -> EntryDomainResult<Vec<Entry>> {
        Err(MemorizDomainError::NotImplemented)
    }

    fn create_entry(&self, entry: Entry) -> EntryDomainResult<Option<Entry>> {
        Ok(self.create_entry(entry)?)
    }

    fn update_entry(&self, entry: Entry) -> EntryDomainResult<Option<Entry>> {
        Ok(self.update_entry(entry)?)
    }

    fn delete_entry(&self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid) -> EntryDomainResult<()> {
        Ok(self.delete_entry(owner_uuid, uuid)?)
    }

    fn archive_entry(&self, entry: Entry) -> EntryDomainResult<Option<Entry>> {
        Ok(self.archive_entry(entry)?)
    }

    fn undo_archive_entry(&self, entry: Entry) -> EntryDomainResult<Option<Entry>> {
        Ok(self.undo_archive_entry(entry)?)
    }

    fn create_board(&self, board: Board) -> EntryDomainResult<Option<Board>> {
        Ok(self.create_board(board)?)
    }
    fn update_board(&self, board: Board) -> EntryDomainResult<Option<Board>> {
        Ok(self.update_board(board)?)
    }
    fn delete_board(&self, board: &Board) -> EntryDomainResult<()> {
        Ok(self.delete_board(board)?)
    }
}
