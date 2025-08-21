use crate::{
    fsio::list_current_dir,
    param::{ExecMode, Param},
};
use colored::Colorize;
use rust_i18n::t;
use std::{collections::HashMap, fs, path::PathBuf};
use unicode_display_width::width;

pub(crate) fn execute(param: &Param) {
    let paths = list_current_dir(param);
    match param.get_exec_mode() {
        ExecMode::DryRun => print_replace_result(paths, param),
        ExecMode::Exec => exec_replace(paths, param),
    }
}

fn print_replace_result(paths: Vec<PathBuf>, param: &Param) {
    if paths.is_empty() {
        println!("{}", t!("result.no.found").red());
        return;
    }

    let str_from = t!("label.from");
    let str_to = t!("label.to");
    let str_status = t!("label.status");

    let str_from_len = width(&str_from) as usize;
    let str_to_len = width(&str_to) as usize;
    let str_status_len = width(&str_status) as usize;

    let str_from_orig_len = str_from.len();
    let str_to_orig_len = str_to.len();
    let str_status_orig_len = str_status.len();

    let str_from_adjust = str_from_orig_len - str_from_len;
    let str_to_adjust = str_to_orig_len - str_to_len;
    let str_status_adjust = str_status_orig_len - str_status_len;

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
            output_vec.push((from_target, to_result, t!("status.dup").to_string(), false));
            conflect_count += 1;
        } else {
            existed.insert(to_result.clone(), 0);
            output_vec.push((from_target, to_result, t!("status.ok").to_string(), true));
        }
    });
    let max_left_len = output_vec
        .iter()
        .fold(0, |acc, x| acc.max(x.0.len()))
        .max(str_from_len);
    let max_right_len = output_vec
        .iter()
        .fold(0, |acc, x| acc.max(x.1.len()))
        .max(str_to_len);
    let max_status_len = output_vec
        .iter()
        .fold(0, |acc, x| acc.max(width(&x.2) as usize))
        .max(str_status_len);
    // 4 = 4 * "|", 6 = 3 * 2 * ' '
    let len = max_left_len + max_right_len + max_status_len + 4 + 6;
    let separator = vec!['-'; len].iter().collect::<String>();
    println!("{separator}");
    println!(
        "| {} | {} | {} |",
        fill_adjust(&str_from.to_string(), max_left_len, str_from_adjust).green(),
        fill_adjust(&str_to.to_string(), max_right_len, str_to_adjust).green(),
        fill_adjust(&str_status.to_string(), max_status_len, str_status_adjust).green(),
    );
    println!("{separator}");
    output_vec.iter().for_each(|x| {
        let status = fill_adjust(&x.2, max_status_len, x.2.len() - width(&x.2) as usize);
        println!(
            "| {} | {} | {} |",
            fill(&x.0, max_left_len),
            fill(&x.1, max_right_len),
            if x.3 { status.green() } else { status.red() },
        )
    });
    println!("{separator}");
    if conflect_count == 0 {
        println!("{}", t!("result.dryrun.suc").green());
    } else {
        println!(
            "{}",
            t!("result.dryrun.fail",conflect_count=> conflect_count).red()
        );
    }
}

fn fill(source: &String, max: usize) -> String {
    format!("{source:<max$}").to_string()
}

fn fill_adjust(source: &String, max: usize, adjust: usize) -> String {
    let new_max = max - adjust;
    fill(source, new_max)
}

fn exec_replace(paths: Vec<PathBuf>, param: &Param) {
    if paths.is_empty() {
        println!("{}", t!("result.no.found").red());
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
                "{}",
                t!("result.exec.log", 
                   file1=> from_path.file_name().unwrap().to_str().unwrap(),
                   file2=> to_path.file_name().unwrap().to_str().unwrap()),
            );
            fs::rename(from_path, to_path).unwrap();
        }
    } else {
        print_replace_result(paths, param);
    }
}
