use serde::Deserialize;

pub mod categories;
pub mod docs;

#[derive(Debug, Deserialize)]
pub struct QueryPagination {
    page: Option<u64>,
    page_size: Option<u64>,
}
