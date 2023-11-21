use env_logger::Env;
use eyre::Result;

use lib::{bootstrap, infrastructure::EnvConfig};

const DEFAULT_LOG_FILTER: &str = "warn,actix_web=info,lib=debug";

#[actix_web::main]
async fn main() -> Result<()> {
    setup_logging();

    let config = EnvConfig::get()?;

    bootstrap(config).await
}

fn setup_logging() {
    let env = Env::default().default_filter_or(DEFAULT_LOG_FILTER);

    env_logger::Builder::from_env(env).init();
}
