use ftx_api::api_response_structs::{account::Position, wallet::Balance};

pub fn get_usd_balance(balances: &Vec<Balance>) -> Option<Balance> {
    for balance in balances {
        if balance.coin == "USD" {
            return Some(balance.clone());
        }
    }
    return None;
}

pub fn holds_positions(positions: &Vec<Position>) -> bool {
    for position in positions {
        if position.size != 0.0 {
            return true;
        }
    }
    return false;
}

pub fn holds_balances(balances: &Vec<Balance>) -> bool {
    for balance in balances {
        if balance.total > 0.0 && balance.coin != "USD" {
            return true;
        }
    }
    return false;
}
