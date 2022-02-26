use execute_data::ExecuteData;
use ftx_api::api_payload_structs::user_data::UserData;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod account_information;
mod execute_data;
mod heuristics;
mod market_data_manipulators;
mod trade_execution;
mod trade_execution_helpers;

#[tokio::main]
async fn main() {
    let executor = setup().await;
    execute(executor).await;
}

async fn execute(executor: ExecuteData) {
    if !executor.strategy_active {
        match (executor.market_to_enter_name, executor.market_to_enter_funding_rate) {
            (Some(name), Some(rate)) => {
                let message = format!(
                    "Activating strategy: no active strategies found and potential strategy available ({}: {})",
                    name, rate
                );
                trade_execution::activate_strategy(name, executor.usd_balance, &executor.user_data, message).await;
            }
            _ => {
                let message = format!("No strategy active and no potential strategy available, no modifications done");
                trade_execution::no_action(message);
            }
        }
    } else {
        if heuristics::is_position_desireable(&executor.account).await {
            let message = format!("Current strategy desireable, no modifications done");
            trade_execution::no_action(message);
        } else {
            let message = format!("Disbling strategy: current strategy not desireable, exiting positions");
            trade_execution::exit_positions(
                executor.balances,
                &executor.account.positions,
                &executor.user_data,
                message,
            )
            .await;
            match (executor.market_to_enter_name, executor.market_to_enter_funding_rate) {
                (Some(name), Some(rate)) => {
                    let message = format!("Activating strategy: Entering new strategy ({}: {})", name, rate);
                    trade_execution::activate_strategy(name, executor.usd_balance, &executor.user_data, message).await;
                }
                _ => println!("No new strategy to enter"),
            }
        }
    }
}

fn get_user_info() -> UserData {
    let reader = BufReader::new(File::open("./settings.txt").expect("Cannot open file.txt"));
    let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let user_data = UserData {
        key: lines.remove(0),
        secret: lines.remove(0),
        sub_account: lines.remove(0),
        min_vol: lines.remove(0).parse().unwrap(),
    };

    return user_data;
}

async fn setup() -> ExecuteData {
    let user_data = get_user_info();
    let mutual_markets = market_data_manipulators::get_mutual_markets(user_data.min_vol).await;
    let market_to_enter = heuristics::find_market_to_enter(mutual_markets).await;
    let balances = ftx_api::wallet::balances(&user_data).await.unwrap().result;
    let account = ftx_api::account::get_account_information(&user_data)
        .await
        .unwrap()
        .result;
    let usd_balance = account_information::get_usd_balance(&balances).unwrap();
    let strategy_active =
        account_information::holds_balances(&balances) || account_information::holds_positions(&account.positions);

    return ExecuteData {
        user_data,
        account,
        strategy_active,
        balances,
        usd_balance,
        market_to_enter_name: market_to_enter.0,
        market_to_enter_funding_rate: market_to_enter.1,
    };
}
