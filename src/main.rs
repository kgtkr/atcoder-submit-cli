#![recursion_limit = "1024"]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate html5ever;
extern crate reqwest;
use clap::{App, Arg};

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
}
