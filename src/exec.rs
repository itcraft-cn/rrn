use crate::{
    fsio::list_current_dir,
    param::{ExecMode, Param},
};
use std::{fs, path::PathBuf};

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
        output_vec.push((from_target, to_result));
    });
    let max_left_len = output_vec.iter().fold(0, |acc, x| acc.max(x.0.len()));
    let max_right_len = output_vec.iter().fold(0, |acc, x| acc.max(x.1.len()));
    let len = max_left_len + max_right_len + 3 + 4;
    let separator = vec!['-'; len].iter().collect::<String>();
    println!("{separator}");
    println!(
        "| {} | {} |",
        fill(&String::from("from"), max_left_len),
        fill(&String::from("to"), max_right_len)
    );
    println!("{separator}");
    output_vec.iter().for_each(|x| {
        println!(
            "| {} | {} |",
            fill(&x.0, max_left_len),
            fill(&x.1, max_right_len)
        )
    });
    println!("{separator}");
    println!("This is dryrun. Execute with '-x' to execute.");
}

fn fill(source: &String, max: usize) -> String {
    format!("{source:<max$}").to_string()
}

fn exec_replace(paths: Vec<PathBuf>, param: &Param) {
    if paths.is_empty() {
        println!("no files/dirs found!");
        return;
    }
    let from_regex = param.get_from_regex();
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
        println!(
            "Move \"{}\" => \"{}\"",
            x.file_name().unwrap().to_str().unwrap(),
            to_file.file_name().unwrap().to_str().unwrap(),
        );
        fs::rename(x, to_file).unwrap();
    });
}
