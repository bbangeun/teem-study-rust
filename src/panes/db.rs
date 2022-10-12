use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::MyError, panes::Pane};

pub async fn get_pane(client: &Client, page_id: i32) -> Result<Pane, MyError> {

    let _stmt = include_str!("../../sql/get_pane.sql");
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&page_id])
        .await?
        .iter()
        .map(|row| Pane::from_row_ref(row).unwrap())
        .collect::<Vec<Pane>>()
        .pop()
        .ok_or(MyError::NotFound)
}

pub async fn add_pane(client: &Client, pane_info: Pane) -> Result<Pane, MyError> {

    let _stmt = include_str!("../../sql/add_pane.sql");
    let _stmt = _stmt.replace("$table_fields", &Pane::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &pane_info.page_id,
                &pane_info.structure,
            ],
        )
        .await?
        .iter()
        .map(|row| Pane::from_row_ref(row).unwrap())
        .collect::<Vec<Pane>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}