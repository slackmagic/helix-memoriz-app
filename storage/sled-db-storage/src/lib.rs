use chrono::prelude::*;
use helix_memoriz_domain::core::{board::*, entry::*, label::*};
use helix_memoriz_domain::storage::error::*;
use helix_memoriz_domain::storage::traits::StorageTrait;
use serde_cbor::ser::to_vec;
use sled::Db;
use uuid::Uuid;

//TODO: Define const for table names
const OWNERSHIP_TREE: &str = "OWNERSHIP";
const BOARDS_TREE: &str = "BOARDS";
const ENTRIES_TREE: &str = "ENTRIES";

pub struct SledDbMemorizStorage {
    db: Db,
}

impl SledDbMemorizStorage {
    pub fn new(path: String) -> StorageResult<Self> {
        let db = sled::open(path)?;
        Ok(SledDbMemorizStorage { db })
    }

    fn generate_uuid(&self) -> Uuid {
        Uuid::new_v5(&Uuid::NAMESPACE_URL, Utc::now().to_rfc3339().as_bytes())
    }
}

impl StorageTrait for SledDbMemorizStorage {
    fn get_board(
        &mut self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> StorageResult<Option<Board>> {
        Err(StorageError::NotImplemented)
    }

    fn create_board(&mut self, mut board: Board) -> StorageResult<Board> {
        let boards = self.db.open_tree(BOARDS_TREE)?;
        board.uuid = Some(self.generate_uuid());

        boards.insert(&board.uuid.unwrap().to_string(), to_vec(&board).unwrap())?;
        Ok(board)
    }
    fn get_all_boards(&mut self, owner_uuid: uuid::Uuid) -> StorageResult<Vec<Board>> {
        self.db.open_tree("")?;
        Err(StorageError::NotImplemented)
    }
    fn update_board(&mut self, board: Board) -> StorageResult<Board> {
        Err(StorageError::NotImplemented)
    }
    fn delete_board(&mut self, board: &Board) -> StorageResult<()> {
        Err(StorageError::NotImplemented)
    }

    fn get_all_labels(&mut self) -> StorageResult<Vec<Label>> {
        Err(StorageError::NotImplemented)
    }
    fn create_label(&mut self, label: Label) -> StorageResult<Label> {
        Err(StorageError::NotImplemented)
    }
    fn update_label(&mut self, label: Label) -> StorageResult<Label> {
        Err(StorageError::NotImplemented)
    }
    fn delete_label(&mut self, label: &Label) -> StorageResult<()> {
        Err(StorageError::NotImplemented)
    }

    fn create_entry(&mut self, mut entry: Entry) -> StorageResult<Entry> {
        let entries = self.db.open_tree(ENTRIES_TREE)?;
        entry.uuid = Some(self.generate_uuid());

        let compound_id = &entry.uuid.unwrap().to_string();
        entries.insert(&compound_id, to_vec(&entry).unwrap())?;
        Ok(entry)
    }
    fn update_entry(&mut self, entry: Entry) -> StorageResult<Entry> {
        Err(StorageError::NotImplemented)
    }
    fn delete_entry(&mut self, owner_uuid: uuid::Uuid, uuid: uuid::Uuid) -> StorageResult<()> {
        Err(StorageError::NotImplemented)
    }
    fn get_entry(
        &mut self,
        owner_uuid: uuid::Uuid,
        uuid: uuid::Uuid,
    ) -> StorageResult<Option<Entry>> {
        Err(StorageError::NotImplemented)
    }

    fn get_all_entries(&mut self, owner_uuid: uuid::Uuid) -> StorageResult<Vec<Entry>> {
        Err(StorageError::NotImplemented)
    }

    fn get_all_entries_by_board(
        &mut self,
        owner_uuid: uuid::Uuid,
        board_uuid: uuid::Uuid,
    ) -> StorageResult<Vec<Entry>> {
        Err(StorageError::NotImplemented)
    }
}
