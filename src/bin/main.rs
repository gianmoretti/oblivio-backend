use loco_rs::cli;
use migration::Migrator;
use oblivio_loco_be::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App, Migrator>().await
}
