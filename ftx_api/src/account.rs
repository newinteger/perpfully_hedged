use crate::{
    api_payload_structs::user_data::UserData,
    api_response_structs::{account::Account, http::Response},
    helpers, method_senders, FTX_REST_ENDPOINT,
};

pub async fn get_account_information(account_information: &UserData) -> Option<Response<Account>> {
    let time: String = helpers::get_current_time().to_string();
    let method = "GET";
    let request_path = format!("{}", "/api/account");
    let api_address = format!("{}{}", FTX_REST_ENDPOINT, "account");
    let message = format!("{}{}{}", time, method, request_path);
    let signed_message = helpers::get_hmac_signed_message(&account_information.secret, message);

    let res = method_senders::send_authenticated_get_request(
        &api_address,
        &time,
        &signed_message,
        account_information,
    )
    .await;

    match res {
        Ok(o) => {
            return Some(o.json::<Response<Account>>().await.unwrap());
        }
        Err(_) => {
            return None;
        }
    }
}
