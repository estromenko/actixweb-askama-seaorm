use sea_orm::DatabaseConnection;

pub struct AppData {
    pub db: DatabaseConnection,
}
