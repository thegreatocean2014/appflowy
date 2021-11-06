use crate::{
    entities::doc::{CreateDocParams, Doc, DocIdentifier, UpdateDocParams},
    errors::DocError,
    services::{doc::doc_initial_string, server::DocumentServerAPI},
};
use flowy_infra::future::ResultFuture;

pub struct DocServerMock {}

impl DocumentServerAPI for DocServerMock {
    fn create_doc(&self, _token: &str, _params: CreateDocParams) -> ResultFuture<(), DocError> {
        ResultFuture::new(async { Ok(()) })
    }

    fn read_doc(&self, _token: &str, params: DocIdentifier) -> ResultFuture<Option<Doc>, DocError> {
        let doc = Doc {
            id: params.doc_id,
            data: doc_initial_string(),
            rev_id: 0,
            base_rev_id: 0,
        };
        ResultFuture::new(async { Ok(Some(doc)) })
    }

    fn update_doc(&self, _token: &str, _params: UpdateDocParams) -> ResultFuture<(), DocError> {
        ResultFuture::new(async { Ok(()) })
    }
}
