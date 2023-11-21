pub mod adapters;
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod shared;

use eyre::Result;

pub async fn bootstrap(config: infrastructure::EnvConfig) -> Result<()> {
    infrastructure::http_server::start(config).await
}
