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
    Version,
    Invalid,
}

#[derive(Debug)]
pub(crate) struct Param {
    from: Option<String>,
    from_regex: Option<Regex>,
    to: Option<String>,
    target: TargetMode,
    exec: ExecMode,
}
impl Param {
    fn new(map: &HashMap<String, String>) -> Self {
        Self {
            from: map
                .get("from")
                .or(Some(&String::from("")))
                .map(|x| x.to_string()),
            from_regex: None,
            to: map
                .get("to")
                .or(Some(&String::from("")))
                .map(|x| x.to_string()),
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
        if check_if_not_exist(&self.from) {
            eprintln!("from not specified");
            return true;
        }
        if check_if_not_exist(&self.to) {
            eprintln!("to not specified");
            return true;
        }
        false
    }

    fn compile_regex(&mut self) {
        self.from_regex
            .replace(to_regex(self.from.as_ref().unwrap().clone()));
    }

    pub(crate) fn get_from_regex(&self) -> &Regex {
        self.from_regex.as_ref().unwrap()
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

fn check_if_not_exist(opt: &Option<String>) -> bool {
    opt.is_none() || opt.as_ref().unwrap().to_string().is_empty()
}

pub(crate) fn args2param() -> Result<Param, ParseArgFailure> {
    let args = env::args().collect::<Vec<String>>();
    let mut opts = Parser::new(&args, "vhf:t:dx");
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
                    Opt('v', None) => ("version".to_string(), "true".to_string()),
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
    if map.contains_key("version") {
        println!(env!("CARGO_PKG_VERSION"));
        return Err(ParseArgFailure::Version);
    }
    let mut param = Param::new(&map);
    if param.invalid() {
        return Err(ParseArgFailure::Invalid);
    }
    param.compile_regex();
    Ok(param)
}

pub(crate) fn print_help() {
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
    eprintln!("\t-h, output help message");
    eprintln!("\t-v, output version info");
    eprintln!("------------------------------------------------------");
}
