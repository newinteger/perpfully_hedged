use array_tool::vec::Intersect;
use ftx_api::{self, api_response_structs::markets::Market};

use regex::Regex;

pub async fn get_mutual_markets(min_vol: f64) -> Vec<String> {
    let markets: Vec<_> = ftx_api::markets::markets().await.unwrap().result;
    let spot_markets = sift_spot_markets(markets.iter());
    let perpetual_markets = sift_perpetual_markets(markets.iter());
    let mutual_markets = sift_mutual_markets(spot_markets, perpetual_markets, min_vol);

    return mutual_markets;
}

fn sift_mutual_markets(spot_markets: Vec<Market>, perpetual_markets: Vec<Market>, min_vol: f64) -> Vec<String> {
    let perps: Vec<String> = perpetual_markets
        .into_iter()
        .filter(|perp| perp.volumeUsd24h > min_vol)
        .map(|perp| perp.name.split("-").next().unwrap().to_string())
        .collect();
    let spots: Vec<String> = spot_markets
        .into_iter()
        .filter(|spot| spot.volumeUsd24h > min_vol)
        .map(|spot| spot.name.split("/").next().unwrap().to_string())
        .collect();
    let valid = spots.intersect(perps);

    return valid;
}

fn sift_perpetual_markets(markets: core::slice::Iter<Market>) -> Vec<Market> {
    let re = Regex::new(r"(-PERP)").unwrap();
    let mut valid: Vec<Market> = Vec::new();

    for market in markets {
        if market.r#type == "future" && re.is_match(&market.name) {
            valid.push(market.clone());
        }
    }
    return valid;
}

fn sift_spot_markets(markets: core::slice::Iter<Market>) -> Vec<Market> {
    let reg_bad = Regex::new(r"(BULL|BEAR|HEDGE|HALF|VOL|USDT)").unwrap();
    let reg_good = Regex::new(r"USD").unwrap();
    let mut valid: Vec<Market> = Vec::new();

    for market in markets {
        if market.r#type == "spot" && !reg_bad.is_match(&market.name) && reg_good.is_match(&market.name) {
            valid.push(market.clone());
        }
    }

    return valid;
}
