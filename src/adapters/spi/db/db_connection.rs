use eyre::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

pub async fn establish_connection(connection_string: &String) -> Result<DatabaseConnection> {
    let db = Database::connect(connection_string).await?;

    Migrator::up(&db, None).await?;

    Ok(db)
}
