pub mod request {
  use serde::{Serialize, Deserialize};

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  #[serde(rename = "request")]
  pub struct Login {
    #[serde(rename = "Username")]
    pub username: String,
    pub password_type: u8,
    #[serde(rename = "Password")]
    pub password: String
  }

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  #[serde(rename = "request")]
  pub struct SMSGet {
    #[serde(rename = "PageIndex")]
    pub page_index: u8,
    #[serde(rename = "ReadCount")]
    pub read_count: u8,
    #[serde(rename = "BoxType")]
    pub box_type: u8,
    #[serde(rename = "SortType")]
    pub sort_type: u8,
    #[serde(rename = "Ascending")]
    pub ascending: u8,
    #[serde(rename = "UnreadPreferred")]
    pub unread_preferred: u8,
  }

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  #[serde(rename = "request")]
  pub struct SMSSend {
    #[serde(rename = "Index")]
    pub index : i32,
    #[serde(rename = "Phones")]
    pub phones : String,
    #[serde(rename = "Sca")]
    pub sca : String,
    #[serde(rename = "Content")]
    pub content : String,
    #[serde(rename = "Length")]
    pub length : u32,
    #[serde(rename = "Reserved")]
    pub reserved : u8,
    #[serde(rename = "Date")]
    pub date : String,
  }

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  pub struct Phones {
    #[serde(rename = "Phone")]
    pub items: Vec<String>,
  }

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  #[serde(rename = "request")]
  pub struct SMSRead {
    #[serde(rename = "Index")]
    pub index : i32,
  }
}

pub mod response {
  use serde::{Serialize, Deserialize};

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  #[serde(rename = "response")]
  pub struct SMSGet{
    #[serde(rename = "Count")]
    pub count : i32,
    #[serde(rename = "Messages")]
    pub messages: Messages,
  }

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  pub struct Messages{
    #[serde(rename = "Message", default)]
    pub items: Vec<Message>,
  }

  #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
  pub struct Message {
    #[serde(rename = "Smstat")]
    pub smstat: i8,
    #[serde(rename = "Index")]
    pub index: i32,
    #[serde(rename = "Phone")]
    pub phone: String,
    #[serde(rename = "Content")]
    pub content: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Sca")]
    pub sca: String,
    #[serde(rename = "SaveType")]
    pub savetype: u8,
    #[serde(rename = "Priority")]
    pub priority: u8,
    #[serde(rename = "SmsType")]
    pub smstype: u8,
  }

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  #[serde(rename = "response")]
  pub struct SessionToken {
    #[serde(rename = "SesInfo")]
    pub session: String,
    #[serde(rename = "TokInfo")]
    pub token: String,
  }

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  #[serde(rename = "response")]
  pub struct SMSGetEmpty{
    #[serde(rename = "Count")]
    pub count : i32,
  }

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  #[serde(rename = "response")]
  pub struct Success {
    #[serde(rename = "$value")]
    pub value: String,
  }

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  #[serde(rename = "response")]
  pub struct Failed {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "message")]
    pub message: String,
  }

  #[derive(Debug, Serialize, Deserialize, PartialEq)]
  #[serde(untagged)]
  pub enum Responses{
    Success(Success),
    Failed(Failed),
    SMSGet(SMSGet),
    SMSEmpty(SMSGetEmpty)
  }
}