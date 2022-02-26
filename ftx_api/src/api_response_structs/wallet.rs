use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance{
    pub coin: String,
    pub free: f64,
    pub spotBorrow: f64,
    pub total: f64,
    pub usdValue: f64,
    pub availableWithoutBorrow: f64
}