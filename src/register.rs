
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Register{
    pub machine_id:Vec<String>,
    pub expire_date:i64,
    pub expire_string:String,
    pub sign_date:i64,
    pub sign_string: String,
    pub order_id:String,
}






/*{
"machine_id": [],
"date_time": 11111,
"sign_date": "",
"order_id": ""
}*/