mod exec;
mod fsio;
mod param;

use crate::{
    exec::execute,
    param::{print_help, ParseArgFailure},
};

fn main() {
    let rs_param = param::args2param();
    if let Ok(param) = rs_param {
        execute(&param);
    } else {
        match rs_param.err().unwrap() {
            ParseArgFailure::Help | ParseArgFailure::Version => (),
            ParseArgFailure::Invalid => {
                eprintln!("Params error");
                print_help();
            }
        }
    }
}
