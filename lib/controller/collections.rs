use super::util::GROUP_COLLECTIONS;
use super::Controller;
use super::{errors::Result, util::build_key};
use crate::models::{Collection, CollectionRequest};
use chrono::Local;

impl Controller {
    pub async fn create_collection(&self, collection: CollectionRequest) -> Result<Collection> {
        let collection = Collection {
            id: xid::new().to_string(),
            created_at: Local::now(),
            name: collection.name,
        };

        let serialized = serde_json::to_vec(&collection)?;

        self.persistence
            .set(
                &build_key(&[GROUP_COLLECTIONS, &collection.id]),
                &serialized,
                self.object_lifetime,
            )
            .await?;

        Ok(collection)
    }

    pub async fn get_collection(&self, id: &str) -> Result<Option<Collection>> {
        Ok(self
            .persistence
            .get(&build_key(&[GROUP_COLLECTIONS, id]))
            .await?
            .map(|v| serde_json::from_slice(&v))
            .transpose()?)
    }

    pub async fn delete_collection(&self, id: &str) -> Result<()> {
        let vals = self
            .persistence
            .list(&build_key(&[GROUP_COLLECTIONS, id]))
            .await?;

        let object_keys: Vec<&str> = vals.iter().map(|(k, _)| k.as_str()).collect();

        self.persistence.delete(&object_keys).await?;

        self.persistence
            .delete(&[&build_key(&[GROUP_COLLECTIONS, id])])
            .await?;

        Ok(())
    }
}
