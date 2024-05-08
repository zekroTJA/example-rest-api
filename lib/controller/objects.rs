use super::errors::Error;
use super::errors::Result;
use super::util::build_key;
use super::util::GROUP_COLLECTIONS;
use super::util::GROUP_OBJECTS;
use super::Controller;
use crate::models::Update;
use crate::models::{Object, ObjectRequest};
use chrono::Local;

impl Controller {
    pub async fn create_object(&self, collection: &str, object: ObjectRequest) -> Result<Object> {
        self.check_collection(collection).await?;

        let object = Object {
            id: xid::new().to_string(),
            created_at: Local::now(),
            name: object.name,
            data: object.data,
        };

        let serialized = serde_json::to_vec(&object)?;

        self.persistence
            .set(
                &build_key(&[GROUP_OBJECTS, collection, &object.id]),
                &serialized,
                self.object_lifetime,
            )
            .await?;

        Ok(object)
    }

    pub async fn set_object(
        &self,
        collection: &str,
        id: &str,
        new_object: <Object as Update>::With,
    ) -> Result<Object> {
        let Some(mut object) = self.get_object(collection, id).await? else {
            return Err(Error::ObjectNotFound);
        };

        object.update(new_object);

        let serialized = serde_json::to_vec(&object)?;

        self.persistence
            .set(
                &build_key(&[GROUP_OBJECTS, collection, id]),
                &serialized,
                self.object_lifetime,
            )
            .await?;

        Ok(object)
    }

    pub async fn get_object(&self, collection: &str, id: &str) -> Result<Option<Object>> {
        self.check_collection(collection).await?;

        Ok(self
            .persistence
            .get(&build_key(&[GROUP_OBJECTS, collection, id]))
            .await?
            .map(|v| serde_json::from_slice(&v))
            .transpose()?)
    }

    pub async fn delete_object(&self, collection: &str, id: &str) -> Result<()> {
        self.check_collection(collection).await?;

        self.persistence
            .delete(&[&build_key(&[GROUP_OBJECTS, collection, id])])
            .await?;

        Ok(())
    }

    pub async fn list_objects(&self, collection: &str) -> Result<Vec<Object>> {
        self.check_collection(collection).await?;

        let kvs = self
            .persistence
            .list(&build_key(&[GROUP_OBJECTS, collection, "*"]))
            .await?;

        let mut res = Vec::with_capacity(kvs.len());

        for (_, val) in kvs {
            let obj = serde_json::from_slice(&val)?;
            res.push(obj);
        }

        Ok(res)
    }

    async fn check_collection(&self, collection: &str) -> Result<()> {
        let collection_exists = self
            .persistence
            .exists(&build_key(&[GROUP_COLLECTIONS, collection]))
            .await?;

        if collection_exists {
            Ok(())
        } else {
            Err(Error::CollectionNotFound)
        }
    }
}
