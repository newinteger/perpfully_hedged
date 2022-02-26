use crate::{FTX_REST_ENDPOINT, method_senders, helpers, api_payload_structs::{self, orders::PlaceOrder}};

pub async fn place_order(
    account_information:&api_payload_structs::user_data::UserData, parameters:PlaceOrder){
    let time:String = helpers::get_current_time().to_string();
    let method = "POST";
    let request_path = "/api/orders";
    let api_address = format!("{}{}",FTX_REST_ENDPOINT,"orders");
    let pp = serde_json::to_string(&parameters).unwrap();
    let message = format!("{}{}{}{}",time,method,request_path,pp);
    let signed_message = helpers::get_hmac_signed_message(&account_information.secret,message);

    let _res = method_senders::send_authenticated_post_request(&api_address, pp, &time, &signed_message, &account_information).await;
}