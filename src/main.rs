mod exec;
mod fsio;
mod param;

use crate::{
    exec::execute,
    param::{print_help, ParseArgFailure},
};
use rust_i18n::{i18n, t};

i18n!("locales", fallback = "en");

fn main() {
    let rs_param = param::args2param();
    if let Ok(param) = rs_param {
        execute(&param);
    } else {
        match rs_param.err().unwrap() {
            ParseArgFailure::Help | ParseArgFailure::Version => (),
            ParseArgFailure::Invalid => {
                eprintln!("{}", t!("param.parse.error"));
                print_help();
            }
        }
    }
}
