use cookie::Cookie;
use reqwest;
use std::collections::HashMap;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

mod auth {
  error_chain!{
      errors{
          Auth
      }
  }
}

mod scrap {
  error_chain!{
      errors{
          Parse
      }

      foreign_links {
          Net(::reqwest::Error);
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

#[derive(Debug, RustcDecodable, RustcEncodable, PartialEq)]
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

  fn to_cookie(&self) -> ::hyper::header::Cookie {
    let mut cookie = ::hyper::header::Cookie::new();
    cookie.append("_session", self.session.to_string());
    cookie.append("_kick_id", self.kick_id.to_string());
    cookie.append("_issue_time", self.issue_time.to_string());
    cookie.append("_user_id", self.user_id.to_string());
    cookie.append("_user_name", self.user_name.to_string());
    cookie
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

#[derive(Debug, RustcDecodable, RustcEncodable, PartialEq)]
pub struct Task {
  abc: String,
  name: String,
  id: i32,
}

pub fn get_tasks(contest: &str, user: &User) -> scrap::Result<Vec<Task>> {
  let client = reqwest::Client::new();
  extern crate select;
  let mut headers = ::hyper::header::Headers::new();
  headers.set(user.to_cookie());
  let body = client
    .get(&format!(
      "https://{}.contest.atcoder.jp/assignments",
      contest
    ))
    .headers(headers)
    .send()?
    .text()?;

  let doc = Document::from(body.as_ref());
  doc
    .find(Name("table").child(Name("tbody")).child(Name("tr")))
    .map(|tr| {
      let tds = tr.find(Name("td")).collect::<Vec<_>>();
      let abc = tds.get(0)?.text();
      let name = tds.get(1)?.text();
      let id = tds
        .get(4)?
        .find(Name("a"))
        .nth(0)?
        .attr("href")?
        .chars()
        .skip(16)
        .collect::<String>()
        .parse::<i32>()
        .ok()?;

      Some(Task {
        abc: abc,
        name: name,
        id: id,
      })
    })
    .map(|r| r.ok_or(scrap::Error::from_kind(scrap::ErrorKind::Parse)))
    .collect::<Result<_, _>>()
}
