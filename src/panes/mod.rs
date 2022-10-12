use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

pub mod handlers;
mod db;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "panes")]
pub struct Pane {
    pub page_id: i32,
    pub structure: String,
}
