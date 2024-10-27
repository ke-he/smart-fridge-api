use sqlx::{query, PgPool, Row};

pub struct ItemService;

impl ItemService {
    pub async fn get_item(&self, db_service: &PgPool, id: &i32) -> String {
        let result = query("SELECT name FROM item WHERE id = $1")
            .bind(id)
            .fetch_one(db_service)
            .await;

        match result {
            Ok(row) => row.get("name"),
            Err(_) => "Not Found".to_string(),
        }
    }
}
