#[macro_use]
extern crate serde_derive;

use async_trait::async_trait;
use helix_memoriz_domain::core::{board::*, entry::*, label::*};
use helix_memoriz_domain::storage::error::*;
use helix_memoriz_domain::storage::traits::SearchEngineTrait;
use meilisearch_sdk::{client::*, document::*, search::*};
use uuid;

#[derive(Serialize, Deserialize, Debug)]
struct IndexedEntry {
    uuid: uuid::Uuid,
    title: String,
    content: String,
    owner_uuid: uuid::Uuid,
}

impl IndexedEntry {
    fn from_entry(entry: &Entry) -> IndexedEntry {
        IndexedEntry {
            uuid: entry.uuid.unwrap(),
            title: entry.title.to_owned(),
            content: entry.content.as_ref().unwrap().to_owned(),
            owner_uuid: entry.owner.unwrap(),
        }
    }
}

impl Document for IndexedEntry {
    type UIDType = uuid::Uuid;

    fn get_uid(&self) -> &Self::UIDType {
        &self.uuid
    }
}

pub struct MsMemorizSearchEngine {
    index: String,
    client: Client,
}

impl MsMemorizSearchEngine {
    pub fn new(
        index: String,
        host: String,
        port: u16,
        token: String,
    ) -> SearchEngineResult<MsMemorizSearchEngine> {
        let url = format!("{}:{}", host.to_string(), port.to_string());
        Ok(MsMemorizSearchEngine {
            index: index,
            client: Client::new(url, token),
        })
    }
}

#[async_trait]
impl SearchEngineTrait for MsMemorizSearchEngine {
    async fn index_entry(&self, entry: &Entry) -> SearchEngineResult<()> {
        let index = self.client.get_or_create(&self.index).await?;
        index
            .add_documents(&[IndexedEntry::from_entry(entry)], Some("uuid"))
            .await?;
        Ok(())
    }

    async fn search_entries(
        &self,
        owner_uuid: uuid::Uuid,
        query: String,
    ) -> SearchEngineResult<Vec<uuid::Uuid>> {
        let index = self.client.get_or_create(&self.index).await?;
        let search_results = index
            .search()
            .with_query(&query)
            .execute::<IndexedEntry>()
            .await
            .unwrap()
            .hits;

        let mut ret: Vec<uuid::Uuid> = Vec::new();
        for search_result in search_results {
            ret.push(search_result.result.uuid);
        }

        Ok(ret)
    }
}
