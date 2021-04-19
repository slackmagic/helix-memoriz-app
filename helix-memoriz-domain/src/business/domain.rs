use crate::business::error::EntryDomainResult;
use crate::business::error::MemorizDomainError;
use crate::business::traits::DomainTrait;
use crate::core::board::Board;
use crate::core::entry::Entry;
use crate::core::label::Label;
use crate::storage::traits::StorageTrait;
use async_trait::async_trait;
use std::boxed::Box;

pub struct MemorizDomain {
    storage: Box<dyn StorageTrait>,
}

impl MemorizDomain {
    pub fn new(storage: Box<dyn StorageTrait>) -> Self {
        MemorizDomain { storage }
    }
}

#[async_trait]
impl DomainTrait for MemorizDomain {
    async fn get_all_entries(
        &self,
        owner_uuid: uuid::Uuid,
        archived_filter: Option<bool>,
    ) -> EntryDomainResult<Vec<Entry>> {
        let entries = self.storage.get_all_entries(owner_uuid).await?;
        let filtered_entries = match archived_filter {
            Some(filter) => entries
                .into_iter()
                .filter(|entry| entry.archived == filter)
                .collect(),
            None => entries,
        };

        Ok(filtered_entries)
    }

    async fn get_all_entries_by_board(
        &self,
        owner_uuid: uuid::Uuid,
        board_uuid: uuid::Uuid,
        archived_filter: Option<bool>,
    ) -> EntryDomainResult<Vec<Entry>> {
        let entries = self
            .storage
            .get_all_entries_by_board(owner_uuid, board_uuid)
            .await?;

        let filtered_entries = match archived_filter {
            Some(filter) => entries
                .into_iter()
                .filter(|entry| entry.archived == filter)
                .collect(),
            None => entries,
        };

        Ok(filtered_entries)
    }

    async fn get_board(
        &self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> EntryDomainResult<Board> {
        Ok(self.storage.get_board(owner_uuid, uuid).await?)
    }

    async fn get_all_boards(&self, owner_uuid: uuid::Uuid) -> EntryDomainResult<Vec<Board>> {
        Ok(self.storage.get_all_boards(owner_uuid).await?)
    }

    async fn get_entry(
        &self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> EntryDomainResult<Entry> {
        Ok(self.storage.get_entry(owner_uuid, uuid).await?)
    }

    async fn search(
        &self,
        owner_uuid: uuid::Uuid,
        content: Option<String>,
        board_uuid: Option<uuid::Uuid>,
        labels: Option<String>,
        archived_filter: Option<bool>,
    ) -> EntryDomainResult<Vec<Entry>> {
        Err(MemorizDomainError::NotImplemented)
    }

    async fn create_entry(&self, entry: Entry) -> EntryDomainResult<Entry> {
        Ok(self.storage.create_entry(entry).await?)
    }

    async fn update_entry(&self, entry: Entry) -> EntryDomainResult<Entry> {
        Ok(self.storage.update_entry(entry).await?)
    }

    async fn delete_entry(
        &self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> EntryDomainResult<()> {
        Ok(self.storage.delete_entry(owner_uuid, uuid).await?)
    }

    async fn archive_entry(
        &self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> EntryDomainResult<Entry> {
        let mut entry = self.get_entry(owner_uuid, uuid).await?;
        entry.archived = true;
        Ok(self.storage.update_entry(entry).await?)
    }

    async fn undo_archive_entry(
        &self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> EntryDomainResult<Entry> {
        let mut entry = self.get_entry(owner_uuid, uuid).await?;
        entry.archived = false;
        Ok(self.storage.update_entry(entry).await?)
    }

    async fn create_board(&self, board: Board) -> EntryDomainResult<Board> {
        Ok(self.storage.create_board(board).await?)
    }
    async fn update_board(&self, board: Board) -> EntryDomainResult<Board> {
        Ok(self.storage.update_board(board).await?)
    }
    async fn delete_board(&self, board: &Board) -> EntryDomainResult<()> {
        Ok(self.storage.delete_board(board).await?)
    }
}
