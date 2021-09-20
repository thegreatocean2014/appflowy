use crate::{
    errors::DocError,
    services::{doc_cache::OpenedDocumentCache, server::construct_doc_server},
};

use crate::{
    entities::doc::{ApplyChangesetParams, CreateDocParams, Doc, QueryDocParams, SaveDocParams},
    errors::internal_error,
    services::{doc_controller::DocController, ws_document::WsDocument},
};


use diesel::SqliteConnection;
use flowy_database::ConnectionPool;
use parking_lot::RwLock;
use std::{sync::Arc};

pub trait DocumentUser: Send + Sync {
    fn user_dir(&self) -> Result<String, DocError>;
    fn user_id(&self) -> Result<String, DocError>;
    fn token(&self) -> Result<String, DocError>;
}

pub struct FlowyDocument {
    controller: Arc<DocController>,
    ws: Arc<RwLock<WsDocument>>,
    cache: Arc<OpenedDocumentCache>,
}

impl FlowyDocument {
    pub fn new(user: Arc<dyn DocumentUser>, ws: Arc<RwLock<WsDocument>>) -> FlowyDocument {
        let server = construct_doc_server();
        let cache = Arc::new(OpenedDocumentCache::new());
        let controller = Arc::new(DocController::new(server.clone(), user.clone()));
        Self { controller, cache, ws }
    }

    pub fn create(&self, params: CreateDocParams, conn: &SqliteConnection) -> Result<(), DocError> {
        let _ = self.controller.create(params, conn)?;
        Ok(())
    }

    pub fn delete(&self, params: QueryDocParams, conn: &SqliteConnection) -> Result<(), DocError> {
        let _ = self.cache.close(&params.doc_id)?;
        let _ = self.controller.delete(params.into(), conn)?;
        Ok(())
    }

    pub async fn open(&self, params: QueryDocParams, pool: Arc<ConnectionPool>) -> Result<Doc, DocError> {
        let doc = match self.cache.is_opened(&params.doc_id) {
            true => {
                let data = self.cache.read_doc(&params.doc_id).await?;
                Doc { id: params.doc_id, data }
            },
            false => {
                let doc = self.controller.open(params, pool).await?;
                let _ = self.cache.open(&doc.id, doc.data.clone())?;
                doc
            },
        };

        Ok(doc)
    }

    pub async fn update(&self, params: SaveDocParams, pool: Arc<ConnectionPool>) -> Result<(), DocError> {
        let _ = self.controller.update(params, &*pool.get().map_err(internal_error)?)?;
        Ok(())
    }

    pub async fn apply_changeset(&self, params: ApplyChangesetParams) -> Result<Doc, DocError> {
        let _ = self
            .cache
            .mut_doc(&params.id, |doc| {
                let _ = doc.apply_changeset(params.data.clone())?;
                Ok(())
            })
            .await?;

        let data = self.cache.read_doc(&params.id).await?;
        let doc = Doc { id: params.id, data };
        Ok(doc)
    }
}
