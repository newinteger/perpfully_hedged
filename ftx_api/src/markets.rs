use crate::{api_response_structs::{markets::Market, http::Response}, FTX_REST_ENDPOINT};

pub async fn markets() -> Option<Response<Vec<Market>>>{
    let api_address = format!("{}{}", FTX_REST_ENDPOINT, "markets");
    let res = reqwest::get(api_address).await;

    match res{
        Ok(o) =>{
            return Some(o.json::<Response<Vec<Market>>>().await.unwrap());
        } 
        Err(_) => {
            return None;
        }
    }
}

pub async fn get_single_market(market:String) -> Option<Response<Market>>{
    let api_address = format!("{}{}{}", FTX_REST_ENDPOINT, "markets/",market);
    let res = reqwest::get(api_address).await;

    match res{
        Ok(o) =>{
            return Some(o.json::<Response<Market>>().await.unwrap());
        } 
        Err(_) => {
            return None;
        }
    }
}