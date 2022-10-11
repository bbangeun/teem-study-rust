use std::alloc::System;
use actix_web::{Error, HttpResponse, web};
use deadpool_postgres::{Client, Pool};

use crate::{errors::MyError, panes::Pane};
use crate::panes::db;

pub async fn add_pane(
    pane: web::Json<Pane>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {

    let pane_info: Pane = pane.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_pane = db::add_pane(&client, pane_info).await?;

    Ok(HttpResponse::Ok().json(new_pane))
}