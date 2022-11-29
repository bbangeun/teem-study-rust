use serde::{Deserialize, Serialize};
use async_graphql::{EmptyMutation, EmptySubscription, Schema, SimpleObject, ComplexObject};
use tokio_pg_mapper_derive::PostgresMapper;

mod model;
pub use model::PanelQuery;

pub type PanelSchema = Schema<PanelQuery, EmptyMutation, EmptySubscription>;

#[derive(SimpleObject, PostgresMapper, Deserialize, Serialize, Debug)]
#[graphql(complex)]
#[pg_mapper(table = "panes")]
pub struct Panel {
    page_id: i32,
    structure: String
}

#[ComplexObject]
impl Panel {}