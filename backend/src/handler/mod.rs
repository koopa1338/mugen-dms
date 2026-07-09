use serde::Deserialize;

pub mod auth;
pub mod collections;
pub mod documents;

#[derive(Debug, Deserialize)]
pub struct QueryPagination {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct QueryCollection {
    pub collection_id: Option<String>,
}
