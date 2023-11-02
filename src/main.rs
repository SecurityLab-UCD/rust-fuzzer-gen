mod transformers;

use quote::ToTokens;
use std::env;
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

/// add println!() to the beginning of the fuzz_target! macro
/// the modified code and the test functions
fn add_report(code: &str) -> (String, Vec<String>) {
    let mut ast = parse_file(&code).expect("Error parsing file");
    let mut transformer = ReportTransformer {
        test_fns: Vec::new(),
    };
    transformer.visit_file_mut(&mut ast);

    return (ast.into_token_stream().to_string(), transformer.test_fns);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: cargo run <path to repo root>");
    }

    let repo_root = Path::new(&args[1]);
    let fuzz_path = repo_root.join("fuzz");
    let fuzz_bak = repo_root.join("fuzz-bak");
    let tests_gen = repo_root.join("tests-gen");

    Command::new("cp")
        .arg("-r")
        .arg(fuzz_path.as_path())
        .arg(fuzz_bak.as_path())
        .output()
        .unwrap();

    Command::new("mkdir")
        .arg("-p")
        .arg(tests_gen.as_path())
        .output()
        .unwrap();

    let fuzzers = get_fuzzers(fuzz_path.to_str().unwrap());

    for fuzzer in fuzzers {
        println!("Processing fuzzer: {}", fuzzer);

        // read the fuzzers and add println!() to the beginning of the fuzz_target! macro
        let code = fs::read_to_string(&fuzzer).expect("Error reading file");
        let (new_code, test_fns) = add_report(&code);
        fs::write(&fuzzer, new_code).expect("Error writing file");

        // use the body of the fuzz_target! macro to generate test functions
        let path = Path::new(&fuzzer);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let tests_gen_file = tests_gen.join(file_name);
        fs::write(tests_gen_file, test_fns.join("\n")).expect("Error writing file");
    }
}
