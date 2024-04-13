#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::models::_entities::asset_designateds::{self, Entity as AssetDesignatedEntity};
use crate::models::_entities::assets::{ActiveModel, Entity, Model};
use crate::models::_entities::designateds::{Entity as DesignatedEntity, Model as DesignatedModel};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub name: String,
    pub description: String,
    pub category: String,
}

// Define a new struct to hold both Asset and its associated Designated entities
#[derive(Debug, Serialize)]
pub struct AssetWithDesignated {
    pub asset: Model,
    pub designated_list: Vec<DesignatedModel>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.name = Set(self.name.clone());
        item.description = Set(self.description.clone());
        item.category = Set(self.category.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Json<Vec<Model>>> {
    let assets = Entity::find().all(&ctx.db).await?;
    format::json(assets)
}

pub async fn enriched_list(
    State(ctx): State<AppContext>,
) -> Result<Json<Vec<AssetWithDesignated>>> {
    let assets = Entity::find().all(&ctx.db).await?;
    let mut assets_with_designated = Vec::new();

    for asset in assets {
        let designated_list = AssetDesignatedEntity::find()
            .filter(asset_designateds::Column::AssetId.eq(asset.id))
            .all(&ctx.db)
            .await?;

        let mut designated_names = Vec::new();
        for designated in designated_list {
            let designated_entity = DesignatedEntity::find_by_id(designated.id)
                .one(&ctx.db)
                .await?;

            designated_names.push(designated_entity.unwrap());
        }

        let asset_with_designated = AssetWithDesignated {
            asset: asset,
            designated_list: designated_names,
        };

        assets_with_designated.push(asset_with_designated);
    }

    format::json(assets_with_designated)
}

pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Json<Model>> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

/*
async fn add(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Json<Model>> {
  // we only want to make sure it exists
  let _current_user = crate::models::users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

  // next, update
  // homework/bonus: make a comment _actually_ belong to user (user_id)
  let mut item = ActiveModel {
      ..Default::default()
  };
  params.update(&mut item);
  let item = item.insert(&ctx.db).await?;
  format::json(item)
}
*/
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
        .prefix("assets")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
        .add("/enriched", get(enriched_list))
}
