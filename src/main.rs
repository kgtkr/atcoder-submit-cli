#![recursion_limit = "1024"]
#[macro_use]
extern crate clap;
extern crate cookie;
#[macro_use]
extern crate error_chain;
extern crate hyper;
extern crate reqwest;
extern crate scraper;
use clap::{App, Arg};
use cookie::Cookie;

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
            Auth(::auth::Error,::auth::ErrorKind);
        }
    }
}

fn main() {
    let app = App::new("atcoder-submit-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .author("tkr <kgtkr.jp@gmail.com>")
        .about("AtCoderにコマンドラインから提出")
        .arg(
            Arg::with_name("user")
                .help("ユーザーID")
                .long("user")
                .short("u")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("pass")
                .help("パスワード")
                .long("pass")
                .short("p")
                .takes_value(true)
                .required(true),
        );

    let matches = app.get_matches();

    let user = matches.value_of("user").unwrap();
    let pass = matches.value_of("pass").unwrap();

    println!("{:?}", login(user, pass));
}

fn login(user: &str, pass: &str) -> login::Result<String> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::RedirectPolicy::none())
        .build()?;
    let mut res = client
        .post("https://practice.contest.atcoder.jp/login")
        .form(&[("name", user), ("password", pass)])
        .send()?;
    let body = res.text()?;
    Ok(res.headers()
        .get::<reqwest::header::SetCookie>()
        .ok_or(auth::Error::from_kind(auth::ErrorKind::Auth))?
        .iter()
        .filter_map(|x| Cookie::parse_encoded(x.to_string()).ok())
        .find(|x| x.name() == "_session")
        .ok_or(auth::Error::from_kind(auth::ErrorKind::Auth))?
        .value()
        .to_string())
}
