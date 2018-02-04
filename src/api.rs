use cookie::Cookie;
use reqwest;

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

pub fn login(user: &str, pass: &str) -> login::Result<String> {
  let client = reqwest::Client::builder()
    .redirect(reqwest::RedirectPolicy::none())
    .build()?;
  let res = client
    .post("https://practice.contest.atcoder.jp/login")
    .form(&[("name", user), ("password", pass)])
    .send()?;
  Ok(
    res
      .headers()
      .get::<reqwest::header::SetCookie>()
      .ok_or(auth::Error::from_kind(auth::ErrorKind::Auth))?
      .iter()
      .filter_map(|x| Cookie::parse_encoded(x.to_string()).ok())
      .find(|x| x.name() == "_session")
      .ok_or(auth::Error::from_kind(auth::ErrorKind::Auth))?
      .value()
      .to_string(),
  )
}
