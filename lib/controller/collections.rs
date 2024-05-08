use super::errors::Error;
use super::util::GROUP_COLLECTIONS;
use super::Controller;
use super::{errors::Result, util::build_key};
use crate::models::{Collection, CollectionRequest, Update};
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

    pub async fn update_collection(
        &self,
        id: &str,
        new_collection: <Collection as Update>::With,
    ) -> Result<Collection> {
        let Some(mut collection) = self.get_collection(id).await? else {
            return Err(Error::CollectionNotFound);
        };

        collection.update(new_collection);
        let serialized = serde_json::to_vec(&collection)?;
        self.persistence
            .set(
                &build_key(&[GROUP_COLLECTIONS, id]),
                &serialized,
                self.object_lifetime,
            )
            .await?;

        Ok(collection)
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
