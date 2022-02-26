use std::collections::HashMap;

use ftx_api::api_response_structs::{account::Account, futures::PerpetualFutureFundingRate};

pub async fn is_position_desireable(account_ref: &Account) -> bool {
    let account = account_ref.clone();
    let position_name = match account.positions.into_iter().find(|p| p.size > 0.0) {
        Some(p) => p.future,
        _ => {
            panic!("Expected position but none found.");
        }
    };
    let positions_stats = ftx_api::futures::stats(&position_name).await.unwrap().result;
    let position_future_funding = positions_stats.nextFundingRate;

    if position_future_funding > 0.0 {
        return true;
    } else {
        return false;
    }
}

pub async fn find_market_to_enter(markets: Vec<String>) -> (Option<String>, Option<f64>) {
    let funding_rates = ftx_api::futures::funding_rates().await.unwrap().result;
    let mut grouped_funding_rates: HashMap<String, Vec<PerpetualFutureFundingRate>> = HashMap::new();

    // Keep only futures that we already are interested in and group funding info in groups per market
    for funding_rate in funding_rates {
        if markets.iter().any(|m| funding_rate.future.starts_with(m)) {
            let group = grouped_funding_rates
                .entry(funding_rate.future.clone())
                .or_insert(vec![]);
            group.push(funding_rate);
        }
    }

    /*
    Naive heuristic to determine if market is suitable to use for hedging.
    Heuristic: If the future market has had positive funding for the past 3 funding payments (3 hours in case of FTX) it is suitable.
    Returns the market that the latest biggest funding payment.
    */
    let mut candidates: Vec<(String, f64)> = Vec::new();
    for key in grouped_funding_rates.keys() {
        for market_group in grouped_funding_rates.get(key) {
            let market_stats = ftx_api::futures::stats(key).await.unwrap().result;
            let expected_funding_rate = market_stats.nextFundingRate;

            if expected_funding_rate < 0.0 {
                break;
            }

            let mut positive_funding_streak = 0;

            for market_data in market_group {
                if market_data.rate > 0.0 {
                    positive_funding_streak = positive_funding_streak + 1;
                } else {
                    break;
                }
            }

            if positive_funding_streak == market_group.len() {
                candidates.push((key.split("-").next().unwrap().to_string(), expected_funding_rate));
            }
        }
    }
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    if candidates.len() == 0 {
        return (None, None);
    } else {
        let best = candidates.remove(0);
        return (Some(best.0), Some(best.1));
    }
}
