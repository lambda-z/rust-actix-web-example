
use core::ops::DerefMut;
use std::ops::Deref;
use log::info;
use mongodb::{bson, Client, Database};
use futures_util::StreamExt;

pub(crate) struct Mongo {
    pub(crate) uri: String,
    pub(crate) db_name: String,
    pub(crate) client: Option<Client>,
    pub(crate) db: Option<Database>
}

impl Mongo {
    pub async fn new(&mut self, uri: String, db_name: String) -> &Self {

        Self {
            uri,
            db_name,
            client: None,
            db: None
        };
        &*self.init().await
    }

    pub(crate) async fn init(&mut self) -> &Self {
        info!("mongo uri: {:?}", self.uri);
        self.client = Option::from(
            Client::with_uri_str(&self.uri).await.unwrap()
        );
        self.db = Option::from(
            self.client.as_ref().unwrap().database(&self.db_name)
        );
        self
    }

    pub(crate) async fn insert_one(&self, coll: &str, doc: bson::Document) {
        let collection = self.db.as_ref().unwrap().collection(coll);
        collection.insert_one(doc, None).await.unwrap();
    }

    pub(crate) async fn find_one(&self, coll: &str, filter: bson::Document) -> bson::Document {
        let collection = self.db.as_ref().unwrap().collection(coll);
        collection.find_one(filter, None).await.unwrap().unwrap()
    }

}

impl Deref for Mongo {
    type Target = ();

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl DerefMut for Mongo {
    fn deref_mut(&mut self) -> &mut () {
        self
    }
}