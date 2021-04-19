use async_trait::async_trait;
use chrono::prelude::*;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use helix_memoriz_domain::core::{board::*, entry::*, label::*};
use helix_memoriz_domain::storage::error::*;
use helix_memoriz_domain::storage::traits::StorageTrait;
use tokio_postgres::tls::NoTls;
use uuid;

pub struct PgDbMemorizStorage {
    pub pool: Pool,
}

impl PgDbMemorizStorage {
    pub fn new(
        database: String,
        host: String,
        port: u16,
        user: String,
        password: String,
    ) -> StorageResult<PgDbMemorizStorage> {
        let mut cfg = Config::new();
        cfg.dbname = Some(database);
        cfg.host = Some(host);
        cfg.port = Some(port);
        cfg.user = Some(user);
        cfg.password = Some(password);
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        Ok(PgDbMemorizStorage {
            pool: cfg.create_pool(NoTls).unwrap(),
        })
    }
}

#[async_trait]
impl StorageTrait for PgDbMemorizStorage {
    async fn create_board(&self, mut board: Board) -> StorageResult<Board> {
        board.created_on = Some(Utc::now());
        let query = "
        INSERT INTO memoriz.board
        VALUES (DEFAULT,$1,$2,$3,$4,NULL,$5)
        RETURNING uuid;";

        let client = self.pool.get().await.unwrap();

        let row_inserted = &client
            .query(
                query,
                &[
                    &board.title,
                    &board.data,
                    &board.color,
                    &board.created_on,
                    &board.owner,
                ],
            )
            .await?;

        let row_data = row_inserted.iter().next().unwrap();
        board.uuid = row_data.get("id");
        Ok(board)
    }

    async fn update_board(&self, mut board: Board) -> StorageResult<Board> {
        board.updated_on = Some(Utc::now());

        let query = "
        UPDATE memoriz.board SET (title, data, color, updated_on) 
        = ($2,$3,$4,$5)
        WHERE UUID = $1;";

        let client = self.pool.get().await.unwrap();

        client
            .query(
                query,
                &[
                    &board.uuid,
                    &board.title,
                    &board.data,
                    &board.color,
                    &board.updated_on,
                ],
            )
            .await?;

        Ok(board)
    }

    async fn get_board(&self, uuid: uuid::Uuid, owner_uuid: uuid::Uuid) -> StorageResult<Board> {
        let mut result: StorageResult<Board> = Err(StorageError::AnotherError);

        let query = "
        select *
        from memoriz.board
        where board.owner_ = $1
        and board.uuid = $2;
        ";

        let client = self.pool.get().await.unwrap();

        for row in &client.query(query, &[&uuid, &owner_uuid]).await? {
            result = Ok(Board::new(
                row.get("uuid"),
                row.get("title"),
                row.get("data"),
                row.get("color"),
                row.get("created_on"),
                row.get("updated_on"),
                row.get("owner_"),
            ));
        }

        result
    }

    async fn get_all_boards(&self, owner_uuid: uuid::Uuid) -> StorageResult<Vec<Board>> {
        let mut result: Vec<Board> = Vec::new();

        let query = "
        select *
        from memoriz.board
        where board.owner_ = $1
        order by board.updated_on desc;
        ";

        let client = self.pool.get().await.unwrap();

        for row in &client.query(query, &[&owner_uuid]).await? {
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

    async fn delete_board(&self, board: &Board) -> StorageResult<()> {
        let query = "DELETE FROM memoriz.board WHERE UUID = $1;";
        let client = self.pool.get().await.unwrap();
        &client.execute(query, &[&board.uuid]).await?;
        Ok(())
    }

    async fn create_label(&self, label: Label) -> StorageResult<Label> {
        let query = "
        INSERT INTO memoriz.label
        VALUES ($1,$2,$3);";

        let client = self.pool.get().await.unwrap();

        client
            .query(query, &[&label.id, &label.name, &label.description])
            .await?;

        Ok(label)
    }

    async fn update_label(&self, label: Label) -> StorageResult<Label> {
        let query = "
        UPDATE memoriz.label SET (name, description) 
        = ($2, $3)
        WHERE ID = $1;";

        let client = self.pool.get().await.unwrap();
        client
            .query(query, &[&label.name, &label.description])
            .await?;
        Ok(label)
    }

    async fn delete_label(&self, label: &Label) -> StorageResult<()> {
        let query = "DELETE FROM memoriz.label WHERE ID = $1;";

        let client = self.pool.get().await.unwrap();

        client.query(query, &[&label.id]).await?;
        Ok(())
    }

    async fn get_all_labels(&self) -> StorageResult<Vec<Label>> {
        let mut result: Vec<Label> = Vec::new();

        let query = "
        select *
        from memoriz.label
        where 1=1
        order by name;
        ";

        let client = self.pool.get().await.unwrap();

        for row in client.query(query, &[]).await? {
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

    async fn create_entry(&self, mut entry: Entry) -> StorageResult<Entry> {
        entry.created_on = Some(Utc::now());

        let query = "
        INSERT INTO memoriz.entry
        VALUES (DEFAULT,DEFAULT,$1,$2,$3,$4,false,$5,NULL,$6,$7)
        RETURNING id, uuid;";

        let client = self.pool.get().await.unwrap();

        let row_inserted = client
            .query(
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
            )
            .await?;

        let row_data = row_inserted.iter().next().unwrap();
        entry.id = row_data.get("id");
        entry.uuid = row_data.get("uuid");

        Ok(entry)
    }

    async fn update_entry(&self, mut entry: Entry) -> StorageResult<Entry> {
        entry.updated_on = Some(Utc::now());

        let query = "
        UPDATE memoriz.entry SET (title, content, data, color, archived, updated_on, board_) 
        = ($2,$3,$4,$5,$6,$7,$8)
        WHERE ID = $1;";

        let client = self.pool.get().await.unwrap();

        client
            .query(
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
            )
            .await?;

        //TODO: Manage label

        Ok(entry)
    }

    async fn delete_entry(&self, uuid: uuid::Uuid, owner_uuid: uuid::Uuid) -> StorageResult<()> {
        let query = "DELETE FROM memoriz.entry WHERE UUID = $1 AND owner_=$2;";
        let client = self.pool.get().await.unwrap();
        client.execute(query, &[&uuid, &owner_uuid]).await?;
        Ok(())
    }

    async fn get_entry(&self, uuid: uuid::Uuid, owner_uuid: uuid::Uuid) -> StorageResult<Entry> {
        let mut result: StorageResult<Entry> = Err(StorageError::AnotherError);

        let query = "
        select *
        from memoriz.entry
        where 1=1
        and entry.uuid = $1
        and entry.owner_ = $2;";

        let client = self.pool.get().await.unwrap();

        for row in &client.query(query, &[&uuid, &owner_uuid]).await? {
            result = Ok(Entry::new(
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

        result
    }

    async fn get_all_entries(&self, owner_uuid: uuid::Uuid) -> StorageResult<Vec<Entry>> {
        let mut result: Vec<Entry> = Vec::new();

        let query = "
        select *
        from memoriz.entry
        where entry.owner_ = $1
        and entry.board_ is NULL
        order by entry.updated_on desc;";

        let client = self.pool.get().await.unwrap();

        for row in client.query(query, &[&owner_uuid]).await? {
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

    async fn get_all_entries_by_board(
        &self,
        owner_uuid: uuid::Uuid,
        board_uuid: uuid::Uuid,
    ) -> StorageResult<Vec<Entry>> {
        let mut result: Vec<Entry> = Vec::new();

        let query = "
        select *
        from memoriz.entry
        where entry.owner_ = $1 and entry.board_ = $2
        order by entry.updated_on desc;";

        let client = self.pool.get().await.unwrap();

        for row in client.query(query, &[&owner_uuid, &board_uuid]).await? {
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
