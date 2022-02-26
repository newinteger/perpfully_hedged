pub async fn max_size_for_bid(market: String, bid: f64) -> f64 {
    let market = ftx_api::markets::get_single_market(market).await.unwrap();
    let ask = market.result.ask.unwrap();
    let size = bid / ask;
    return size;
}
