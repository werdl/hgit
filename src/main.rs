mod callee;
mod cmd;

use std::env;
use std::process::Command;
use std::process::exit;


fn bad_arg() -> String {
    return "Incorrect arguments provided".to_string()
}

fn git_hgit(i: String) -> String {
    return i.replace("git", "hgit").replace("Git", "HGit")
}

fn call_git(args: Vec<String>) {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("Failed to execute command");

    // println!("{}\n{}", git_hgit(String::from_utf8(output.stdout).unwrap()), git_hgit(String::from_utf8(output.stderr).unwrap()));
    println!("{:?}", output);

}

fn version_callback(_:Option<String>) -> String {
    return format!("{}, hgit version {}", String::from_utf8(Command::new("git")
    .args(vec!["--version"])
    .output()
    .expect("Failed to execute command").stdout).unwrap().replace("\n", ""), env!("CARGO_PKG_VERSION"));
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let zero = match args.get(1) {
        None => {
            println!("hgit version {}, licensed under the Apache License v2.0", env!("CARGO_PKG_VERSION"));
            exit(-1);
        },
        Some(x) => {
            x
        }
    };
    match zero {
        // by default, just call git, but allow flags like --version
        _ => {
            let mut parser = cmd::start(None);
            parser.add_callback('v', "version", version_callback, true);
            let trimmed = args.get(1..).unwrap().to_vec();
            let result = parser.parse(trimmed.clone(), 0);
            if result.is_empty() {
                println!("{:?}", trimmed.clone());
                call_git(trimmed.clone());
            }
        }
    }

}
