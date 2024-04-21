#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::models::_entities::asset_designateds::{ActiveModel, Column, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub asset_id: i32,
    pub designated_id: i32,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.asset_id = Set(self.asset_id);
        item.designated_id = Set(self.designated_id);
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<Model>>> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[derive(Deserialize)]
pub struct QueryParams {
    asset_id: Option<i32>,
    designated_id: Option<i32>,
}

pub async fn query(
    State(ctx): State<AppContext>,
    Query(params): Query<QueryParams>,
) -> Result<Json<Vec<Model>>> {
    if params.asset_id.is_some() {
        format::json(
            Entity::find()
                .filter(Column::AssetId.eq(params.asset_id))
                .all(&ctx.db)
                .await?,
        )
    } else if params.designated_id.is_some() {
        format::json(
            Entity::find()
                .filter(Column::DesignatedId.eq(params.designated_id))
                .all(&ctx.db)
                .await?,
        )
    } else {
        Err(Error::BadRequest(
            "Please, use a valid request parameter choosing from asset_id and designated_id"
                .to_string(),
        ))
    }
}

pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Json<Model>> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Json<Model>> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<()> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Json<Model>> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("asset_designateds")
        .add("/", get(list))
        .add("/", post(add))
        .add("/query", get(query))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
}
