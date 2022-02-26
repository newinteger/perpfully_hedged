use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaceOrder{
    pub market:String, 
    pub side:String, 
    pub price:Option<f64>,
    pub r#type:String, 
    pub size:f64, 
    pub reduce_only:bool,
    pub ioc:bool, 
    pub post_only:bool, 
    pub client_id:Option<String>,
    pub reject_on_price_band:bool, 
    pub reject_after_ts:Option<f64>
}

impl PlaceOrder {
    //price: None for market orders
    pub fn basic_order(market:String, side:String, price:Option<f64>, r#type:String, size:f64) -> PlaceOrder{
        PlaceOrder{
            market,
            side,
            price,
            r#type,
            size,
            reduce_only: false,
            ioc: false,
            post_only: false,
            client_id: None,
            reject_on_price_band: false,
            reject_after_ts: None
        }
    }
}