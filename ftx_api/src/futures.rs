use crate::{api_response_structs::{futures::{PerpetualFutureStatstic, PerpetualFutureFundingRate, Future}, http::Response}, FTX_REST_ENDPOINT};

pub async fn stats(future_name:&str) -> Option<Response<PerpetualFutureStatstic>>{
    let api_address = format!("{}futures/{}/{}",FTX_REST_ENDPOINT,future_name,"stats");
    let res = reqwest::get(api_address).await;

    match res{
        Ok(o) =>{
            return Some(o.json::<Response<PerpetualFutureStatstic>>().await.unwrap());
        }
        Err(_) => {
            return None;
        }
    }
}

pub async fn funding_rates() -> Option<Response<Vec<PerpetualFutureFundingRate>>> {
    let api_address = format!("{}{}", FTX_REST_ENDPOINT, "funding_rates");
    let res = reqwest::get(api_address).await;

    match res{
        Ok(o) =>{
            return Some(o.json::<Response<Vec<PerpetualFutureFundingRate>>>().await.unwrap());
        } 
        Err(_) => {
            return None;
        }
    }
}

pub async fn futures() -> Option<Response<Vec<Future>>>{
    let api_address = format!("{}{}", FTX_REST_ENDPOINT, "futures");
    let res = reqwest::get(api_address).await;

    match res{
        Ok(o) =>{
            return Some(o.json::<Response<Vec<Future>>>().await.unwrap());
        } 
        Err(_) => {
            return None
        }
    }
}