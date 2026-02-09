// API request/response models
// These are separate from the core domain models to allow API versioning

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTicketRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TicketResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub acceptance_criteria: Vec<AcceptanceCriteriaResponse>,
    pub created_at: String,
    pub updated_at: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptanceCriteriaResponse {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}
