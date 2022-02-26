use crate::{FTX_REST_ENDPOINT, method_senders, helpers, api_payload_structs::user_data::UserData, api_response_structs::{wallet::Balance, http::Response}};

pub async fn balances(account_information:&UserData) -> Option<Response<Vec<Balance>>>{
    let time:String = helpers::get_current_time().to_string();
    let method = "GET";
    let request_path = "/api/wallet/balances";
    let api_address = format!("{}{}",FTX_REST_ENDPOINT,"wallet/balances");
    let message = format!("{}{}{}",time,method,request_path);
    let signed_message = helpers::get_hmac_signed_message(&account_information.secret,message);

    let res = method_senders::send_authenticated_get_request(&api_address, &time,&signed_message, account_information).await;        
    match res{
        Ok(o) =>{
            return Some(o.json::<Response<Vec<Balance>>>().await.unwrap());
        } 
        Err(_) => {
            return None;
        }
    }
}