use ftx_api::{
    api_payload_structs::user_data::UserData,
    api_response_structs::{account::Account, wallet::Balance},
};

pub struct ExecuteData {
    pub user_data: UserData,
    pub account: Account,
    pub strategy_active: bool,
    pub balances: Vec<Balance>,
    pub usd_balance: Balance,
    pub market_to_enter_name: Option<String>,
    pub market_to_enter_funding_rate: Option<f64>,
}
