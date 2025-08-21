mod exec;
mod fsio;
mod param;

use crate::{
    exec::execute,
    param::{print_help, ParseArgFailure},
};
use rust_i18n::{available_locales, i18n, set_locale, t};
use std::env;

i18n!("locales", fallback = "en, zh");

fn main() {
    setup_locale();
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

fn setup_locale() {
    let env_lang = env::var("LANG").unwrap();
    let current_locale = env_lang.split_at(2).0;
    let vec = available_locales!();
    if vec.contains(&current_locale) {
        set_locale(current_locale);
    } else {
        set_locale("en");
    }
}
