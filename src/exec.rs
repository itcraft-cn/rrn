use crate::{
    fsio::list_current_dir,
    param::{ExecMode, Param},
};
use std::{collections::HashMap, fs, path::PathBuf};

const COLOR_RED: &str = "\x1b[1;91m";
const COLOR_GREEN: &str = "\x1b[1;92m";
const COLOR_NONE: &str = "\x1b[0m";

const STR_FROM: &str = "from";
const STR_TO: &str = "to";
const STR_STATUS: &str = "status";

const STR_FROM_LEN: usize = STR_FROM.len();
const STR_TO_LEN: usize = STR_TO.len();
const STR_STATUS_LEN: usize = STR_STATUS.len();

pub(crate) fn execute(param: &Param) {
    let paths = list_current_dir(param);
    match param.get_exec_mode() {
        ExecMode::DryRun => print_replace_result(paths, param),
        ExecMode::Exec => exec_replace(paths, param),
    }
}

fn print_replace_result(paths: Vec<PathBuf>, param: &Param) {
    if paths.is_empty() {
        println!("no files/dirs found!");
        return;
    }
    let mut output_vec = Vec::new();
    let from_regex = param.get_from_regex();
    let mut existed = HashMap::new();
    let mut conflect_count = 0;
    paths.iter().for_each(|x| {
        let from_target = x.file_name().unwrap().to_string_lossy().to_string();
        let to_result = from_regex
            .replace_all(
                x.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .as_str(),
                &param.get_to_pattern(),
            )
            .to_string();
        if existed.contains_key(&to_result) {
            output_vec.push((from_target, to_result, "Duplicated".to_string(), false));
            conflect_count += 1;
        } else {
            existed.insert(to_result.clone(), 0);
            output_vec.push((from_target, to_result, "OK".to_string(), true));
        }
    });
    let max_left_len = output_vec
        .iter()
        .fold(0, |acc, x| acc.max(x.0.len()))
        .max(STR_FROM_LEN);
    let max_right_len = output_vec
        .iter()
        .fold(0, |acc, x| acc.max(x.1.len()))
        .max(STR_TO_LEN);
    let max_status_len = output_vec
        .iter()
        .fold(0, |acc, x| acc.max(x.2.len()))
        .max(STR_STATUS_LEN);
    // 4 = 4 * "|", 6 = 3 * 2 * ' '
    let len = max_left_len + max_right_len + max_status_len + 4 + 6;
    let separator = vec!['-'; len].iter().collect::<String>();
    println!("{separator}");
    println!(
        "| {}{}{} | {}{}{} | {}{}{} |",
        COLOR_GREEN,
        fill(&STR_FROM.to_string(), max_left_len),
        COLOR_NONE,
        COLOR_GREEN,
        fill(&STR_TO.to_string(), max_right_len),
        COLOR_NONE,
        COLOR_GREEN,
        fill(&STR_STATUS.to_string(), max_status_len),
        COLOR_NONE,
    );
    println!("{separator}");
    output_vec.iter().for_each(|x| {
        println!(
            "| {} | {} | {}{}{} |",
            fill(&x.0, max_left_len),
            fill(&x.1, max_right_len),
            if x.3 { COLOR_GREEN } else { COLOR_RED },
            fill(&x.2, max_status_len),
            COLOR_NONE
        )
    });
    println!("{separator}");
    if conflect_count == 0 {
        println!(
            "{}This is dryrun. Execute with '-x' to execute.{}",
            COLOR_GREEN, COLOR_NONE
        );
    } else {
        println!(
            "{}Duplicate detected {conflect_count} files/dirs, please recheck.{}",
            COLOR_RED, COLOR_NONE
        );
    }
}

fn fill(source: &String, max: usize) -> String {
    format!("{source:<max$}").to_string()
}

fn exec_replace(paths: Vec<PathBuf>, param: &Param) {
    if paths.is_empty() {
        println!("{}no files/dirs found!{}", COLOR_RED, COLOR_NONE);
        return;
    }
    let from_regex = param.get_from_regex();
    let mut existed = HashMap::new();
    let mut ready_vec = Vec::new();
    let mut conflect_count = 0;
    paths.iter().for_each(|x| {
        let to_result = from_regex
            .replace_all(
                x.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .as_str(),
                &param.get_to_pattern(),
            )
            .to_string();
        let to_file = x.parent().unwrap().join(&to_result);
        if existed.contains_key(&to_result) {
            conflect_count += 1;
        } else {
            existed.insert(to_result.clone(), 0);
            ready_vec.push((x.clone(), to_file));
        }
    });
    if conflect_count == 0 {
        for (from_path, to_path) in ready_vec {
            println!(
                "Move \"{}\" => \"{}\"",
                from_path.file_name().unwrap().to_str().unwrap(),
                to_path.file_name().unwrap().to_str().unwrap(),
            );
            fs::rename(from_path, to_path).unwrap();
        }
    } else {
        print_replace_result(paths, param);
    }
}
