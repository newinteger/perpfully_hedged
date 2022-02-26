use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response<T> {
    pub success: Option<bool>,
    pub result: T,
}
