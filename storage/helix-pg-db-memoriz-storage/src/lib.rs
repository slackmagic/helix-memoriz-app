use chrono::prelude::*;
use helix_memoriz_domain::core::{board::*, entry::*, label::*};
use helix_memoriz_domain::storage::error::*;
use helix_memoriz_domain::storage::traits::StorageTrait;
use postgres::{Client, NoTls};
use uuid;

pub struct PgDbMemorizStorage {
    pub db_conn: Client,
}

impl PgDbMemorizStorage {
    pub fn new(conn_string: String) -> Self {
        let t_connection: Client = Client::connect(&conn_string, NoTls).unwrap();
        PgDbMemorizStorage {
            db_conn: t_connection,
        }
    }
}

impl StorageTrait for PgDbMemorizStorage {
    fn create_board(&mut self, mut board: Board) -> StorageResult<Board> {
        board.created_on = Some(Utc::now());

        let query = "
        INSERT INTO memoriz.board
        VALUES (DEFAULT,$1,$2,$3,$4,NULL,$5)
        RETURNING uuid;";

        let row_inserted = &self.db_conn.query(
            query,
            &[
                &board.title,
                &board.data,
                &board.color,
                &board.created_on,
                &board.owner,
            ],
        )?;

        let row_data = row_inserted.iter().next().unwrap();
        board.uuid = row_data.get("id");
        Ok(board)
    }

    fn update_board(&mut self, mut board: Board) -> StorageResult<Board> {
        board.updated_on = Some(Utc::now());

        let query = "
        UPDATE memoriz.board SET (title, data, color, updated_on) 
        = ($2,$3,$4,$5)
        WHERE UUID = $1;";

        self.db_conn.query(
            query,
            &[
                &board.uuid,
                &board.title,
                &board.data,
                &board.color,
                &board.updated_on,
            ],
        )?;

        Ok(board)
    }

    fn get_board(
        &mut self,
        uuid: uuid::Uuid,
        owner_uuid: uuid::Uuid,
    ) -> StorageResult<Option<Board>> {
        let mut result: Option<Board> = None;

        let query = "
        select *
        from memoriz.board
        where board.owner_ = $1
        and board.uuid = $2;
        ";

        for row in &self.db_conn.query(query, &[&uuid, &owner_uuid])? {
            result = Some(Board::new(
                row.get("uuid"),
                row.get("title"),
                row.get("data"),
                row.get("color"),
                row.get("created_on"),
                row.get("updated_on"),
                row.get("owner_"),
            ));
        }

        Ok(result)
    }

    fn get_all_boards(&mut self, owner_uuid: uuid::Uuid) -> StorageResult<Vec<Board>> {
        let mut result: Vec<Board> = Vec::new();

        let query = "
        select *
        from memoriz.board
        where board.owner_ = $1
        order by board.updated_on desc;
        ";

        for row in &self.db_conn.query(query, &[&owner_uuid])? {
            let board: Board = Board::new(
                row.get("uuid"),
                row.get("title"),
                row.get("data"),
                row.get("color"),
                row.get("created_on"),
                row.get("updated_on"),
                row.get("owner_"),
            );

            result.push(board);
        }

        Ok(result)
    }

    fn delete_board(&mut self, board: &Board) -> StorageResult<()> {
        let query = "DELETE FROM memoriz.board WHERE UUID = $1;";
        &self.db_conn.execute(query, &[&board.uuid])?;
        Ok(())
    }

    fn create_label(&mut self, label: Label) -> StorageResult<Label> {
        let query = "
        INSERT INTO memoriz.label
        VALUES ($1,$2,$3);";

        self.db_conn
            .query(query, &[&label.id, &label.name, &label.description])?;

        Ok(label)
    }

    fn update_label(&mut self, label: Label) -> StorageResult<Label> {
        let query = "
        UPDATE memoriz.label SET (name, description) 
        = ($2, $3)
        WHERE ID = $1;";
        self.db_conn
            .query(query, &[&label.name, &label.description])?;
        Ok(label)
    }

    fn delete_label(&mut self, label: &Label) -> StorageResult<()> {
        let query = "DELETE FROM memoriz.label WHERE ID = $1;";

        self.db_conn.query(query, &[&label.id])?;
        Ok(())
    }

    fn get_all_labels(&mut self) -> StorageResult<Vec<Label>> {
        let mut result: Vec<Label> = Vec::new();

        let query = "
        select *
        from memoriz.label
        where 1=1
        order by name;
        ";

        for row in self.db_conn.query(query, &[])? {
            let label_item = Label::new(
                row.get("id"),
                row.get("name"),
                row.get("Description"),
                row.get("owner"),
            );
            result.push(label_item);
        }

        Ok(result)
    }

    fn create_entry(&mut self, mut entry: Entry) -> StorageResult<Entry> {
        entry.created_on = Some(Utc::now());

        let query = "
        INSERT INTO memoriz.entry
        VALUES (DEFAULT,DEFAULT,$1,$2,$3,$4,false,$5,NULL,$6,$7)
        RETURNING id, uuid;";

        let row_inserted = self.db_conn.query(
            query,
            &[
                &entry.title,
                &entry.content,
                &entry.data,
                &entry.color,
                &entry.created_on,
                &entry.owner,
                &entry.board,
            ],
        )?;

        let row_data = row_inserted.iter().next().unwrap();
        entry.id = row_data.get("id");
        entry.uuid = row_data.get("uuid");

        Ok(entry)
    }

    fn update_entry(&mut self, mut entry: Entry) -> StorageResult<Entry> {
        entry.updated_on = Some(Utc::now());

        let query = "
        UPDATE memoriz.entry SET (title, content, data, color, archived, updated_on, board_) 
        = ($2,$3,$4,$5,$6,$7,$8)
        WHERE ID = $1;";
        self.db_conn.query(
            query,
            &[
                &entry.id,
                &entry.title,
                &entry.content,
                &entry.data,
                &entry.color,
                &entry.archived,
                &entry.updated_on,
                &entry.board,
            ],
        )?;

        //TODO: Manage label

        Ok(entry)
    }

    fn delete_entry(&mut self, uuid: uuid::Uuid, owner_uuid: uuid::Uuid) -> StorageResult<()> {
        let query = "DELETE FROM memoriz.entry WHERE UUID = $1 AND owner_=$2;";
        self.db_conn.execute(query, &[&uuid, &owner_uuid])?;
        Ok(())
    }

    fn get_entry(
        &mut self,
        uuid: uuid::Uuid,
        owner_uuid: uuid::Uuid,
    ) -> StorageResult<Option<Entry>> {
        let mut result: Option<Entry> = None;

        let query = "
        select *
        from memoriz.entry
        where 1=1
        and entry.uuid = $1
        and entry.owner_ = $2;";
        for row in &self.db_conn.query(query, &[&uuid, &owner_uuid])? {
            result = Some(Entry::new(
                row.get("id"),
                row.get("uuid"),
                row.get("title"),
                row.get("content"),
                row.get("data"),
                row.get("color"),
                row.get("archived"),
                row.get("created_on"),
                row.get("updated_on"),
                row.get("owner_"),
                None,
                row.get("board_"),
            ));
        }

        Ok(result)
    }

    fn get_all_entries(&mut self, owner_uuid: uuid::Uuid) -> StorageResult<Vec<Entry>> {
        let mut result: Vec<Entry> = Vec::new();

        let query = "
        select *
        from memoriz.entry
        where entry.owner_ = $1
        and entry.board_ is NULL
        order by entry.updated_on desc;";

        for row in self.db_conn.query(query, &[&owner_uuid]).unwrap() {
            let entry: Entry = Entry::new(
                row.get("id"),
                row.get("uuid"),
                row.get("title"),
                row.get("content"),
                row.get("data"),
                row.get("color"),
                row.get("archived"),
                row.get("created_on"),
                row.get("updated_on"),
                row.get("owner_"),
                None,
                row.get("board_"),
            );

            result.push(entry);
        }

        Ok(result)
    }

    fn get_all_entries_by_board(
        &mut self,
        owner_uuid: uuid::Uuid,
        board_uuid: uuid::Uuid,
    ) -> StorageResult<Vec<Entry>> {
        let mut result: Vec<Entry> = Vec::new();

        let query = "
        select *
        from memoriz.entry
        where entry.owner_ = $1 and entry.board_ = $2
        order by entry.updated_on desc;";

        for row in self
            .db_conn
            .query(query, &[&owner_uuid, &board_uuid])
            .unwrap()
        {
            let entry: Entry = Entry::new(
                row.get("id"),
                row.get("uuid"),
                row.get("title"),
                row.get("content"),
                row.get("data"),
                row.get("color"),
                row.get("archived"),
                row.get("created_on"),
                row.get("updated_on"),
                row.get("owner_"),
                None,
                row.get("board_"),
            );

            result.push(entry);
        }

        Ok(result)
    }
}
