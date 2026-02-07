use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

/// Get board configuration and state
pub async fn get_board() -> (StatusCode, Json<Value>) {
    // TODO: Implement with storage backend
    (
        StatusCode::OK,
        Json(json!({
            "name": "Default Board",
            "columns": [
                {"name": "New", "status": "new"},
                {"name": "Open", "status": "open"},
                {"name": "In Progress", "status": "in_progress"},
                {"name": "Pending", "status": "pending"},
                {"name": "Review", "status": "review"},
                {"name": "Done", "status": "done"},
                {"name": "Closed", "status": "closed"}
            ],
            "tickets": {}
        })),
    )
}
