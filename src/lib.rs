use sha2::{Digest, Sha256};
use std::str::FromStr;
mod errors;
mod models;

use errors::Errors;

pub fn get_session_token() -> Result<models::response::SessionToken, Errors> {
  let client: reqwest::Client = reqwest::Client::new();
  let url = format!(
    "http://{}/api/webserver/SesTokInfo",
    dotenv::var("SMS_HOST")?
  );
  let mut resp: reqwest::Response = client.get(url.as_str()).header("DNT", 1).send()?;
  let text = resp.text().unwrap();
  let response: models::response::SessionToken = serde_xml_rs::from_str(text.as_str())?;
  Ok(response)
}

pub fn login(
  username: String,
  password: String,
  token: String,
  session: String,
) -> Result<(models::response::SessionToken, models::response::Responses), Errors> {
  let client: reqwest::Client = reqwest::Client::new();
  let url = format!("http://{}/api/user/login", dotenv::var("SMS_HOST")?);
  let password_hashed = prepare_password(username.clone(), password.clone(), token.clone());
  let request = models::request::Login {
    username,
    password: password_hashed,
    password_type: 4,
  };
  let mut resp: reqwest::Response = client
    .post(url.as_str())
    .body(serde_xml_rs::to_string(&request)?)
    .headers(create_headers(
      token.clone(),
      session.clone(),
      "4".to_owned(),
      None,
    )?)
    .send()?;
  let text = resp.text().unwrap();
  let response = serde_xml_rs::from_str(text.as_str())?;
  Ok((get_response_headers(&resp), response))
}

pub fn get_sms(
  token: String,
  session: String,
  unread: bool,
) -> Result<(models::response::SessionToken, models::response::Responses), Errors> {
  let client: reqwest::Client = reqwest::Client::new();
  let url = format!("http://{}/api/sms/sms-list", dotenv::var("SMS_HOST")?);
  let request = models::request::SMSGet {
    page_index: 1,
    read_count: 50,
    box_type: 1,
    sort_type: 0,
    ascending: 0,
    unread_preferred: match unread {
      true => 1,
      false => 0,
    },
  };
  let mut resp: reqwest::Response = client
    .post(url.as_str())
    .body(serde_xml_rs::to_string(&request)?)
    .headers(create_headers(
      token.clone(),
      session.clone(),
      "1".to_owned(),
      None,
    )?)
    .send()?;
  let text: String = resp.text()?;
  let response: models::response::Responses;
  let sms_empty_enum: Option<models::response::SMSGetEmpty> =
    match serde_xml_rs::from_str(text.as_str()) {
      Ok(data) => Some(data),
      Err(_e) => None,
    };
  let sms_get_enum: Option<models::response::SMSGet> = match serde_xml_rs::from_str(text.as_str()) {
    Ok(data) => Some(data),
    Err(_e) => None,
  };
  let failed_enum: Option<models::response::Failed> = match serde_xml_rs::from_str(text.as_str()) {
    Ok(data) => Some(data),
    Err(_e) => None,
  };
  if sms_empty_enum.is_some() && sms_get_enum.is_none() {
    response = models::response::Responses::SMSEmpty(sms_empty_enum.unwrap());
    Ok((get_response_headers(&resp), response))
  } else if sms_get_enum.is_some() {
    response = models::response::Responses::SMSGet(sms_get_enum.unwrap());
    Ok((get_response_headers(&resp), response))
  } else {
    response = models::response::Responses::Failed(failed_enum.unwrap());
    Ok((get_response_headers(&resp), response))
  }
}

pub fn send_sms(
  token: String,
  session: String,
  number: Vec<String>,
  message: String,
) -> Result<(models::response::SessionToken, models::response::Responses), Errors> {
  let client: reqwest::Client = reqwest::Client::new();
  let url = format!("http://{}/api/sms/send-sms", dotenv::var("SMS_HOST")?);
  let request = models::request::SMSSend {
    index: -1,
    phones: convert_vec_to_hashvec(number),
    sca: "".to_owned(),
    content: message.clone(),
    length: message.len() as u32,
    reserved: 1,
    date: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
  };
  let mut resp: reqwest::Response = client
    .post(url.as_str())
    .body(serde_xml_rs::to_string(&request)?)
    .headers(create_headers(
      token.clone(),
      session.clone(),
      "1".to_owned(),
      None,
    )?)
    .send()?;
  let text = resp.text().unwrap();
  let response: models::response::Responses = serde_xml_rs::from_str(text.as_str())?;
  Ok((get_response_headers(&resp), response))
}

pub fn read_sms(
  token: String,
  session: String,
  index: i32,
) -> Result<(models::response::SessionToken, models::response::Responses), Errors> {
  let client: reqwest::Client = reqwest::Client::new();
  let url = format!("http://{}/api/sms/set-read", dotenv::var("SMS_HOST")?);
  let request = models::request::SMSRead { index };
  let mut resp: reqwest::Response = client
    .post(url.as_str())
    .body(serde_xml_rs::to_string(&request)?)
    .headers(create_headers(
      token.clone(),
      session.clone(),
      "1".to_owned(),
      None,
    )?)
    .send()?;
  let text = resp.text().unwrap();
  let response: models::response::Responses = serde_xml_rs::from_str(text.as_str())?;
  Ok((get_response_headers(&resp), response))
}

pub fn delete_sms(
  token: String,
  session: String,
  index: i32,
) -> Result<(models::response::SessionToken, models::response::Responses), Errors> {
  let client: reqwest::Client = reqwest::Client::new();
  let url = format!("http://{}/api/sms/delete-sms", dotenv::var("SMS_HOST")?);
  let request = models::request::SMSRead { index: index };
  let mut resp: reqwest::Response = client
    .post(url.as_str())
    .body(serde_xml_rs::to_string(&request)?)
    .headers(create_headers(
      token.clone(),
      session.clone(),
      "1".to_owned(),
      None,
    )?)
    .send()?;
  let text = resp.text().unwrap();
  let response: models::response::Responses = serde_xml_rs::from_str(text.as_str())?;
  Ok((get_response_headers(&resp), response))
}

/* PRIVATE METHODS */
fn prepare_password(username: String, password: String, token: String) -> String {
  let mashed = username.to_owned() + &sha256_and_b64(password).as_str() + &token;
  sha256_and_b64(mashed)
}

fn sha256_and_b64(item: String) -> String {
  let mut hasher = Sha256::new();
  hasher.input(item);
  let phex = hex::encode(hasher.result());
  base64::encode(&phex)
}

fn create_headers(
  token: String,
  session: String,
  dnt: String,
  encoding: Option<String>,
) -> Result<reqwest::header::HeaderMap, Errors> {
  let mut headers = reqwest::header::HeaderMap::new();
  headers.insert(
    reqwest::header::COOKIE,
    reqwest::header::HeaderValue::from_str(&session)?,
  );
  headers.insert(
    reqwest::header::HeaderName::from_str("__RequestVerificationToken")?,
    reqwest::header::HeaderValue::from_str(&token)?,
  );
  headers.insert(
    reqwest::header::HeaderName::from_str("DNT")?,
    reqwest::header::HeaderValue::from_str(&dnt)?,
  );
  match encoding {
    Some(encoding) => {
      headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_str(&encoding)?,
      );
    }
    None => {}
  };
  Ok(headers)
}

fn get_response_headers(response: &reqwest::Response) -> models::response::SessionToken {
  let headers = models::response::SessionToken {
    session: match response.headers().get("set-cookie") {
      Some(head) => head.to_str().unwrap().to_owned(),
      None => "".to_owned(),
    },
    token: match response.headers().get("__RequestVerificationToken") {
      Some(head) => head.to_str().unwrap().to_owned(),
      None => "".to_owned(),
    },
  };
  headers
}

fn convert_vec_to_hashvec(vec: Vec<String>) -> String {
  let mut hashedvec = Vec::new();
  for elem in vec {
    hashedvec.push(format!("<Phone>{}</Phone>", elem));
  }
  hashedvec.join("")
}
