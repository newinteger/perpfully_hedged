use reqwest::Error;
use reqwest::Response;

use crate::api_payload_structs::user_data::UserData;

pub async fn send_authenticated_get_request(api_address:&str,time:&str,signed_message:&str, account_information:&UserData) -> Result<Response, Error>{
    let client = reqwest::Client::new();
    let res = client.get(api_address)
        .header("FTX-KEY", account_information.key.to_string())
        .header("FTX-TS", time)
        .header("FTX-SIGN", signed_message)
        .header("FTX-SUBACCOUNT", account_information.sub_account.to_string())
        .send().await;
    return res;
}

pub async fn send_authenticated_post_request(api_address:&str, body:String,time:&str,signed_message:&str, account_information:&UserData) -> Result<Response, Error>{
    let client = reqwest::Client::new();
    let res = client.post(api_address)
        .header("FTX-KEY", account_information.key.to_string())
        .header("FTX-TS", time)
        .header("FTX-SIGN", signed_message)
        .header("FTX-SUBACCOUNT", account_information.sub_account.to_string())
        .body(body)
        .send().await;
    return res;
}