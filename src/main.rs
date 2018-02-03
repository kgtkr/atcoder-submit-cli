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

mod errors {
    error_chain! {
        foreign_links {
            RequestError(::reqwest::Error);
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

fn login(user: &str, pass: &str) -> errors::Result<String> {
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
        .unwrap()
        .iter()
        .map(|x| cookie::Cookie::parse(x.to_string()).unwrap())
        .find(|x| x.name() == "_session")
        .unwrap()
        .value()
        .to_string())
}
