use getopt::{Opt, Parser};
use regex::Regex;
use std::{collections::HashMap, env};

#[derive(Debug)]
pub(crate) enum TargetMode {
    File,
    Dir,
}

#[derive(Debug)]
pub(crate) enum ExecMode {
    DryRun,
    Exec,
}

#[derive(Debug)]
pub(crate) enum ParseArgFailure {
    Help,
    Invalid,
}

#[derive(Debug)]
pub(crate) struct Param {
    from: Option<Regex>,
    to: Option<String>,
    target: TargetMode,
    exec: ExecMode,
}
impl Param {
    fn new(map: &HashMap<String, String>) -> Self {
        Self {
            from: Option::from(to_regex(map.get("from").map(|x| x.to_string()).unwrap())),
            to: map.get("to").map(|x| x.to_string()),
            target: match map.get("dir") {
                None => TargetMode::File,
                Some(_) => TargetMode::Dir,
            },
            exec: match map.get("exec") {
                None => ExecMode::DryRun,
                Some(_) => ExecMode::Exec,
            },
        }
    }

    fn invalid(&self) -> bool {
        if self.from.is_none() {
            eprintln!("from not specified");
            return true;
        }
        if self.to.is_none() {
            eprintln!("to not specified");
            return true;
        }
        false
    }

    pub(crate) fn get_from_regex(&self) -> &Regex {
        self.from.as_ref().unwrap()
    }

    pub(crate) fn get_to_pattern(&self) -> String {
        String::from(self.to.as_ref().unwrap())
    }

    pub(crate) fn get_target_mode(&self) -> &TargetMode {
        &self.target
    }

    pub(crate) fn get_exec_mode(&self) -> &ExecMode {
        &self.exec
    }
}

fn to_regex(from_pattern: String) -> Regex {
    match Regex::new(&from_pattern) {
        Ok(regex) => regex,
        Err(_) => panic!("error: {from_pattern} is not regex pattern"),
    }
}

pub(crate) fn args2param() -> Result<Param, ParseArgFailure> {
    let args = env::args().collect::<Vec<String>>();
    let mut opts = Parser::new(&args, "hf:t:dx");
    let mut map = HashMap::new();
    loop {
        let rs = opts.next().transpose();
        if let Ok(opt) = rs {
            let (key, val) = match opt {
                None => break,
                Some(opt) => match opt {
                    Opt('f', Some(val)) => ("from".to_string(), val),
                    Opt('t', Some(val)) => ("to".to_string(), val),
                    Opt('d', None) => ("dir".to_string(), "true".to_string()),
                    Opt('x', None) => ("exec".to_string(), "true".to_string()),
                    Opt('h', None) => ("help".to_string(), "true".to_string()),
                    _ => unreachable!(),
                },
            };
            map.insert(key, val);
        } else {
            continue;
        }
    }
    if map.is_empty() || map.contains_key("help") {
        print_help();
        return Err(ParseArgFailure::Help);
    }
    let param = Param::new(&map);
    if param.invalid() {
        print_help();
        return Err(ParseArgFailure::Invalid);
    }
    Ok(param)
}

fn print_help() {
    eprintln!("------------------------------------------------------");
    eprintln!("rrn\ta rename file / directory tool.");
    eprintln!();
    eprintln!("\t-f <pattern>, necessary: true");
    eprintln!("\t\tfrom pattern");
    eprintln!("\t-t <pattern>, necessary: true");
    eprintln!("\t\tto pattern");
    eprintln!("\t-d, optional, default: none");
    eprintln!("\t\trename directories or files, default is rename files.");
    eprintln!("\t-x, optional, default: dry run");
    eprintln!("\t\texecution the rename process");
    eprintln!("\t-h, optional, default: none");
    eprintln!("\t\toutput help message");
    eprintln!("------------------------------------------------------");
}
