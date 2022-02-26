use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Market{
    pub name: String,
    pub baseCurrency: Option<String>,
    pub quoteCurrency: Option<String>,
    pub quoteVolume24h: Option<f64>,
    pub change1h: f64,
    pub change24h: f64,
    pub changeBod: f64,
    pub highLeverageFeeExempt: bool,
    pub largeOrderThreshold: f64,
    pub minProvideSize: f64,
    pub r#type: String,
    pub underlying: Option<String>,
    pub enabled: bool,
    pub ask: Option<f64>,
    pub bid: Option<f64>,
    pub last: f64,
    pub postOnly: bool,
    pub price: Option<f64>,
    pub priceIncrement: f64,
    pub sizeIncrement: f64,
    pub restricted: bool,
    pub volumeUsd24h: f64
}
