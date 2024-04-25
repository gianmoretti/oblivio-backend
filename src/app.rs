use object_store::gcp::GoogleCloudStorageBuilder;
use std::path::Path;

use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks},
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    storage::{self, drivers::object_store_adapter::ObjectStoreAdapter, Storage},
    task::Tasks,
    worker::{AppWorker, Processor},
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;

use crate::{
    controllers,
    models::_entities::{notes, users},
    tasks,
    workers::downloader::DownloadWorker,
};

pub struct App;
#[async_trait]
impl Hooks for App {
    async fn storage(
        _config: &Config,
        _environment: &Environment,
    ) -> Result<Option<storage::Storage>> {
        return Ok(Some(Storage::single(Box::new(ObjectStoreAdapter::new(
            Box::new(
                GoogleCloudStorageBuilder::new()
                    .with_bucket_name("oblivio-asset-documents")
                    .with_service_account_path(
                        std::env::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap(),
                    )
                    .build()
                    .unwrap(),
            ),
        )))));
    }

    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::asset_designated::routes())
            .prefix("/api")
            .add_route(controllers::auth::routes())
            .add_route(controllers::user::routes())
            .add_route(controllers::notes::routes())
            .add_route(controllers::asset::routes())
            .add_route(controllers::designated::routes())
            .add_route(controllers::asset_document::routes())
    }

    fn connect_workers<'a>(p: &'a mut Processor, ctx: &'a AppContext) {
        p.register(DownloadWorker::build(ctx));
    }

    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
    }

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, users::Entity).await?;
        truncate_table(db, notes::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        db::seed::<notes::ActiveModel>(db, &base.join("notes.yaml").display().to_string()).await?;
        Ok(())
    }
}
