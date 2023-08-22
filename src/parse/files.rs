use crate::parse::handle_parsed_items;
use crate::CallableInfo;
use std::path::{Path, PathBuf};

fn get_file_content(path: &Path) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(content) => Some(content),
        Err(e) => {
            log::error!("Error reading file {:?}: {}", path, e);
            None
        }
    }
}

fn get_parsed_file(content: String) -> Option<syn::File> {
    match syn::parse_file(&content) {
        Ok(file) => Some(file),
        Err(e) => {
            log::error!("Failed to parse rust code from file content: {}", e);
            None
        }
    }
}

fn get_functions_from_parsed_file(parsed_file: syn::File, file_path: PathBuf) -> Vec<CallableInfo> {
    handle_parsed_items(&parsed_file.items, &file_path)
}

fn get_functions_from_src_file(path: PathBuf) -> Option<Vec<CallableInfo>> {
    match get_file_content(&path).and_then(get_parsed_file) {
        Some(parsed) => Some(get_functions_from_parsed_file(parsed, path)),
        None => {
            log::error!("Failed to parse functions from source file {:?}", path);
            None
        }
    }
    // let parsed = get_file_content(&path).and_then(get_parsed_file)?;
    // Some(get_functions_from_parsed_file(parsed))
}

fn get_functions_from_src_files(paths: Vec<PathBuf>) -> Vec<CallableInfo> {
    let mut functions = Vec::new();
    for path in paths {
        match get_functions_from_src_file(path) {
            Some(fns) => functions.extend(fns),
            None => continue,
        }
    }
    functions
}

/// Checks to see if a file is a source file we should read.
/// Not actually positive what I need to check for. For now, just that it's a `.rs` file.
fn file_is_src_file(path: &Path) -> bool {
    match path.is_dir() {
        true => false,
        false => match path.extension() {
            Some(ext) => ext == "rs",
            None => false,
        },
    }
}

fn get_src_files_from_path(path: &Path) -> Vec<PathBuf> {
    let mut src_files = Vec::new();
    if path.is_dir() {
        match std::fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        src_files.extend(get_src_files_from_path(&path));
                    } else if file_is_src_file(&path) {
                        src_files.push(path);
                    }
                }
            }
            Err(e) => {
                log::error!("Error reading directory {:?}: {}", path, e);
            }
        }
    } else if file_is_src_file(path) {
        src_files.push(path.to_path_buf());
    }
    src_files
}

fn get_src_files_from_paths(paths: Vec<&Path>) -> Vec<PathBuf> {
    let mut src_files = Vec::new();

    for path in paths {
        src_files.extend(get_src_files_from_path(path));
    }

    src_files
}

pub fn get_fuctions_from_paths(paths: Vec<&Path>) -> Vec<CallableInfo> {
    let src_files = get_src_files_from_paths(paths);
    if src_files.is_empty() {
        log::warn!("No source files were found in provided paths");
    }
    let functions = get_functions_from_src_files(src_files);
    if functions.is_empty() {
        log::warn!("No functions were able to be parsed from provided paths");
    }
    functions
}
