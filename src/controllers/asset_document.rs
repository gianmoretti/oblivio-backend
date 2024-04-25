#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use std::path::PathBuf;

use crate::models::_entities::asset_documents::{ActiveModel, Entity, Model};
use axum::extract::Multipart;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub mime_type: String,
    pub name: String,
    pub filename: String,
    pub url: Option<String>,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.mime_type = Set(self.mime_type.clone());
        item.name = Set(self.name.clone());
        item.filename = Set(self.filename.clone());
        item.url = Set(self.url.clone());
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

async fn upload_file(
    State(ctx): State<AppContext>,
    mut multipart: Multipart,
) -> Result<Json<String>> {
    let mut file = None;
    while let Some(field) = multipart.next_field().await.map_err(|err| {
        tracing::error!(error = ?err,"could not read multipart");
        Error::BadRequest("could not readd multipart".into())
    })? {
        let file_name = match field.file_name() {
            Some(file_name) => file_name.to_string(),
            _ => return Err(Error::BadRequest("file name not found".into())),
        };

        let content = field.bytes().await.map_err(|err| {
            tracing::error!(error = ?err,"could not read bytes");
            Error::BadRequest("could not read bytes".into())
        })?;

        let path = PathBuf::from("example").join(file_name);
        ctx.storage
            .as_ref()
            .unwrap()
            .upload(path.as_path(), &content)
            .await?;

        file = Some(path);
    }

    file.map_or_else(not_found, |path| {
        let path_string = path.as_path().display().to_string();
        format::json(path_string)
    })
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("asset_documents")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
        .add("/upload", post(upload_file))
}
