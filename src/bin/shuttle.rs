use loco_rs::boot::{create_app, StartMode};
use loco_rs::environment::Environment;
use migration::Migrator;
use oblivio_loco_be::app::App;
use shuttle_runtime::DeploymentMetadata;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://default:UI7GDx0XCtJq@ep-sparkling-shadow-a45jhttd-pooler.us-east-1.aws.neon.tech:5432/verceldb?sslmode=require&currentSchema=public"
    )]
    conn_str: String,
    #[shuttle_runtime::Metadata] _meta: DeploymentMetadata,
) -> shuttle_axum::ShuttleAxum {
    std::env::set_var("DATABASE_URL", conn_str);
    let environment = Environment::Development;
    let boot_result = create_app::<App, Migrator>(StartMode::ServerOnly, &environment)
        .await
        .unwrap();

    let router = boot_result.router.unwrap();
    Ok(router.into())
}
