use cookie::Cookie;
use reqwest;
use std::collections::HashMap;

mod auth {
  error_chain!{
      errors{
          Auth
      }
  }
}

mod login {
  error_chain! {
      foreign_links {
          Net(::reqwest::Error);
      }

      links{
          Auth(super::auth::Error,super::auth::ErrorKind);
      }
  }
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct User {
  issue_time: String,
  kick_id: String,
  session: String,
  user_id: String,
  user_name: String,
}

impl User {
  fn from_cookie(cookie: &::hyper::header::SetCookie) -> auth::Result<User> {
    let cookie_map = cookie
      .iter()
      .filter_map(|x| Cookie::parse_encoded(x.to_string()).ok())
      .map(|x| (x.name().to_string(), x.value().to_string()))
      .collect::<HashMap<_, _>>();
    Ok(User {
      session: cookie_map
        .get("_session")
        .ok_or(auth::Error::from_kind(auth::ErrorKind::Auth))?
        .to_string(),
      kick_id: cookie_map
        .get("_kick_id")
        .ok_or(auth::Error::from_kind(auth::ErrorKind::Auth))?
        .to_string(),
      issue_time: cookie_map
        .get("_issue_time")
        .ok_or(auth::Error::from_kind(auth::ErrorKind::Auth))?
        .to_string(),
      user_id: cookie_map
        .get("_user_id")
        .ok_or(auth::Error::from_kind(auth::ErrorKind::Auth))?
        .to_string(),
      user_name: cookie_map
        .get("_user_name")
        .ok_or(auth::Error::from_kind(auth::ErrorKind::Auth))?
        .to_string(),
    })
  }
}

pub fn login(user: &str, pass: &str) -> login::Result<User> {
  let client = reqwest::Client::builder()
    .redirect(reqwest::RedirectPolicy::none())
    .build()?;
  let res = client
    .post("https://practice.contest.atcoder.jp/login")
    .form(&[("name", user), ("password", pass)])
    .send()?;
  let cookie = res
    .headers()
    .get::<reqwest::header::SetCookie>()
    .ok_or(auth::Error::from_kind(auth::ErrorKind::Auth))?;
  Ok(User::from_cookie(&cookie)?)
}

//pub fn get_tasks(contest: &str, user: User) -> Result<Vec<(String, String)>, ()> {}
