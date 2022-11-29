use async_graphql::{Context, FieldResult, Object};
use deadpool_postgres::{Client, Pool};
use super::{Panel};
use crate::errors::MyError;
use tokio_pg_mapper::FromTokioPostgresRow;

pub struct PanelQuery;

#[Object]
impl PanelQuery {
    async fn hello(&self, _ctx: &Context<'_>) -> String {
        "GraphQL says Hello!".to_string()
    }

    async fn panels(&self, ctx: &Context<'_>) -> FieldResult<Vec<Panel>> {

        let pool = ctx.data::<Pool>().unwrap();
        let client: Client = pool.get().await.map_err(MyError::PoolError)?;

        let query_str = format!("SELECT page_id, structure FROM panes");
        let stmt = client.prepare(&query_str).await.unwrap();

        let result = client
            .query(&stmt, &[])
            .await?
            .iter()
            .map(|row| Panel::from_row_ref(row).unwrap())
            .collect::<Vec<Panel>>()
            ;

        return Ok(result);
    }
}