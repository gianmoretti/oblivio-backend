#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use chrono::NaiveDate;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::designateds::{ActiveModel, Entity, Model};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub birth_place: Option<String>,
    pub residence: Option<String>,
    pub phone_number: Option<String>,
    pub fiscal_code: Option<String>,
    pub color: Option<String>,
    pub image_url: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.email = Set(self.email.clone());
        item.first_name = Set(self.first_name.clone());
        item.last_name = Set(self.last_name.clone());
        item.birth_date = Set(self.birth_date.clone());
        item.birth_place = Set(self.birth_place.clone());
        item.residence = Set(self.residence.clone());
        item.phone_number = Set(self.phone_number.clone());
        item.fiscal_code = Set(self.fiscal_code.clone());
        item.color = Set(self.color.clone());
        item.image_url = Set(self.image_url.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<Model>>> {
    format::json(Entity::find().all(&ctx.db).await?)
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
        .prefix("designateds")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
}
