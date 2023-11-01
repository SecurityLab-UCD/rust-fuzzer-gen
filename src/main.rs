mod transformers;

use quote::ToTokens;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use syn::{parse_file, visit_mut::VisitMut};
use transformers::ReportTransformer;
use walkdir::WalkDir;

/// Get all the fuzzers in the fuzz directory
fn get_fuzzers(fuzz_path: &str) -> Vec<String> {
    let mut fuzzers = Vec::new();
    for entry in WalkDir::new(fuzz_path) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let path_buf = PathBuf::from(path);
            let path_str = path_buf.to_str().unwrap();
            if path_str.ends_with(".rs") {
                fuzzers.push(path_str.to_string());
            }
        }
    }
    return fuzzers;
}

fn add_report(code: &str) -> String {
    let mut ast = parse_file(&code).expect("Error parsing file");
    let mut transformer = ReportTransformer;
    transformer.visit_file_mut(&mut ast);
    return ast.into_token_stream().to_string();
}

fn main() {
    let repo_root = Path::new("data/rust-base64");
    let fuzz_path = repo_root.join("fuzz");
    let fuzz_bak = repo_root.join("fuzz-bak");
    let _ = Command::new("cp")
        .arg("-r")
        .arg(fuzz_path.as_path())
        .arg(fuzz_bak.as_path())
        .output()
        .unwrap();

    let fuzzers = get_fuzzers(fuzz_path.to_str().unwrap());

    for fuzzer in fuzzers {
        let code = fs::read_to_string(&fuzzer).expect("Error reading file");
        let new_code = add_report(&code);
        fs::write(fuzzer, new_code).expect("Error writing file");
    }
}
