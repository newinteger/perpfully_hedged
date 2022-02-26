use crate::trade_execution_helpers;
use ftx_api::{
    api_payload_structs::user_data::UserData,
    api_response_structs::{account::Position, wallet::Balance},
};

pub async fn activate_strategy(market: String, usd_balance: Balance, account_information: &UserData, message: String) {
    //Reducing odds of liquidation caused by price disparities
    let safe_usd_balance = usd_balance.free * 0.9;

    let usd_bid = safe_usd_balance / 2.0;
    let spot_market = format!("{}{}", market, "/USD");
    let future_market = format!("{}{}", market, "-PERP");

    let spot_size = trade_execution_helpers::max_size_for_bid(spot_market.clone(), usd_bid).await;
    let future_size = trade_execution_helpers::max_size_for_bid(future_market.clone(), usd_bid).await;

    make_market_order(account_information, spot_market, "buy", spot_size).await;
    make_market_order(account_information, future_market, "sell", future_size).await;
    println!("{}", message);
}

pub fn no_action(message: String) {
    println!("{}", message);
}

async fn make_market_order(account_information: &UserData, market: String, side: &str, size: f64) {
    let parameters = ftx_api::api_payload_structs::orders::PlaceOrder::basic_order(
        market.to_string(),
        side.to_string(),
        None,
        "market".to_string(),
        size,
    );
    ftx_api::orders::place_order(account_information, parameters).await;
}

pub async fn exit_positions(
    balances: Vec<Balance>,
    positions: &Vec<Position>,
    account_information: &UserData,
    message: String,
) {
    exit_spot_positions(balances, account_information).await;
    exit_future_positions(positions, account_information).await;
    println!("{}", message);
}

async fn exit_spot_positions(balances: Vec<Balance>, account_information: &UserData) {
    for balance in balances {
        let name = balance.coin;
        if name != "USD" {
            let market = format!("{}{}", name, "/USD");
            make_market_order(account_information, market, "sell", balance.total).await;
        }
    }
}

async fn exit_future_positions(positions: &Vec<Position>, account_information: &UserData) {
    for position in positions {
        if position.size > 0.0 {
            make_market_order(account_information, position.future.clone(), "buy", position.size).await;
        }
    }
}
