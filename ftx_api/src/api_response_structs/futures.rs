use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PerpetualFutureStatstic{
    pub volume: f64,
    pub nextFundingRate: f64,
    pub nextFundingTime: String,
    openInterest: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PerpetualFutureFundingRate{
    pub future: String,
    pub rate: f64,
    pub time: String
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Future{
    pub ask: Option<f64>,
    pub bid: Option<f64>,
    pub change1h: f64,
    pub change24h: f64,
    pub changeBod: f64,
    pub volumeUsd24h: f64,
    pub volume: f64,
    pub description: String,
    pub enabled: bool,
    pub expired: bool,
    pub expiry: Option<String>,
    pub index: f64,
    pub imfFactor: f64,
    pub last: Option<f64>,
    pub lowerBound: f64,
    pub mark: f64,
    pub name: String,
    pub openInterest: f64,
    pub openInterestUsd: f64,
    pub perpetual: bool,
    pub positionLimitWeight: f64,
    pub postOnly: bool,
    pub priceIncrement: f64,
    pub sizeIncrement: f64,
    pub underlying: String,
    pub upperBound: f64,
    pub r#type: String
}
