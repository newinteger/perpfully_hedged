use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account{
    pub backstopProvider: bool,
    pub collateral: f64,
    pub freeCollateral: f64,
    pub initialMarginRequirement: f64,
    pub leverage: f64,
    pub liquidating: bool,
    pub maintenanceMarginRequirement: f64,
    pub makerFee: f64,
    pub marginFraction: Option<f64>,
    pub openMarginFraction: Option<f64>,
    pub takerFee: f64,
    pub totalAccountValue: f64,
    pub totalPositionSize: f64,
    pub username: String,
    pub positions: Vec<Position>
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position{
    pub cost: f64,
    pub entryPrice: Option<f64>,
    pub future: String,
    pub initialMarginRequirement: f64,
    pub longOrderSize: f64,
    pub maintenanceMarginRequirement: f64,
    pub netSize: f64,
    pub openSize: f64,
    pub realizedPnl: f64,
    pub shortOrderSize: f64,
    pub side: String,
    pub size: f64,
    pub unrealizedPnl: f64
}