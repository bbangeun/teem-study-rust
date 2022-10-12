use actix_web::{Error, HttpResponse, web};
use deadpool_postgres::{Client, Pool};

use crate::{errors::MyError, panes::Pane};
use crate::panes::db;

pub async fn get_pane(
    page_id: web::Path<i32>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {

    let page_id = page_id.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let pane_info = db::get_pane(&client, page_id).await?;

    Ok(HttpResponse::Ok().json(pane_info))
}

pub async fn add_pane(
    pane: web::Json<Pane>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {

    let pane_info: Pane = pane.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_pane = db::add_pane(&client, pane_info).await?;

    Ok(HttpResponse::Ok().json(new_pane))
}