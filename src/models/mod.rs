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
    pub parent: Option<String>,
    pub blocks: Option<Vec<String>>,
    pub rank: Option<i64>,
    pub effort: Option<u32>,
    pub autonomous: Option<bool>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<String>,
    #[serde(default, skip_serializing_if = "is_zero_i64")]
    pub rank: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<u32>,
    pub autonomous: bool,
}

fn is_zero_i64(n: &i64) -> bool {
    *n == 0
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
