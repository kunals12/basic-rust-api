use sqlx::{Error, MySqlPool};

pub async fn database_connection() -> Result<MySqlPool, Error> {
    MySqlPool::connect("mysql://root:password@localhost:3306/mydb").await
}
