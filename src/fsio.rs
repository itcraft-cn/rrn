use crate::param::{Param, TargetMode};
use regex::Regex;
use std::{
    ffi::OsString,
    fs::{self, ReadDir},
    path::{Path, PathBuf},
};

pub(crate) fn list_current_dir(param: &Param) -> Vec<PathBuf> {
    let rs_data = fs::read_dir(".");
    if let Ok(files_and_dirs) = rs_data {
        let target_mode = param.get_target_mode();
        match target_mode {
            TargetMode::File => list_files(param, files_and_dirs),
            TargetMode::Dir => list_dirs(param, files_and_dirs),
        }
    } else {
        Vec::new()
    }
}

fn list_files(param: &Param, files_and_dirs: ReadDir) -> Vec<PathBuf> {
    list_entry(param, FileChecker::new_checker(), files_and_dirs)
}

fn list_dirs(param: &Param, files_and_dirs: ReadDir) -> Vec<PathBuf> {
    list_entry(param, DirChecker::new_checker(), files_and_dirs)
}

fn list_entry(
    param: &Param,
    checker: Box<dyn DirEntryChecker>,
    files_and_dirs: ReadDir,
) -> Vec<PathBuf> {
    let from_pattern = param.get_from_pattern();
    let from_regex = to_regex(from_pattern);
    files_and_dirs
        .filter(|x| x.is_ok())
        .filter(|x| {
            let name = &x.as_ref().unwrap().file_name();
            checker.check(name) && from_regex.is_match(name.to_str().unwrap())
        })
        .map(|x| x.unwrap().path())
        .collect()
}

fn to_regex(from_pattern: String) -> Regex {
    match Regex::new(&from_pattern) {
        Ok(regex) => regex,
        Err(_) => panic!("error: {from_pattern} is not regex pattern"),
    }
}

trait DirEntryChecker {
    fn check(&self, name: &OsString) -> bool;
}

struct DirChecker {}
impl DirChecker {
    fn new_checker() -> Box<dyn DirEntryChecker> {
        Box::new(Self {})
    }
}
impl DirEntryChecker for DirChecker {
    fn check(&self, name: &OsString) -> bool {
        Path::new(name).is_dir()
    }
}

struct FileChecker {}
impl FileChecker {
    fn new_checker() -> Box<dyn DirEntryChecker> {
        Box::new(Self {})
    }
}
impl DirEntryChecker for FileChecker {
    fn check(&self, name: &OsString) -> bool {
        Path::new(name).is_file()
    }
}
