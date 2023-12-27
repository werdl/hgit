mod callee;
mod cmd;

use std::env;
use std::process::Command;
use std::process::exit;


fn bad_arg(arg: String) -> String {
    return format!("Bad arg - {}", arg);
}

fn call_git(args: Vec<String>) {
    let _ = Command::new("git")
        .args(args)
        .status()
        .expect("Failed to execute command");

}

fn call_str<T: ToString>(cmd: T) {
    let cmd_final: String = cmd.to_string();
    call_git(cmd_final.split_whitespace()
    .map(|s| s.to_string())
    .collect());
}

fn version_callback(_:Option<String>) -> String {
    return format!("{}, hgit version {}", String::from_utf8(Command::new("git")
    .args(vec!["--version"])
    .output()
    .expect("Failed to execute command").stdout).unwrap().replace("\n", ""), env!("CARGO_PKG_VERSION"));
}

fn remove_flags(args: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    for arg in args {
        if arg.chars().nth(0).unwrap() != '-' {
            res.push(arg);
        }
    }

    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let zero = match args.get(1) {
        None => {
            println!("hgit version {}, licensed under the Apache License v2.0", env!("CARGO_PKG_VERSION"));
            exit(-1);
        },
        Some(x) => {
            x
        }
    };
    match zero.as_str() {
        "get" => {
            /*
                * The get command - accepts a provider (default GitHub), then pulls from that source
                * flags: --github (-g), --gitlab (-l)
            */
            
            let mut provider = "github".to_string();

            let mut parser = cmd::start(Some(bad_arg));
            parser.add_option('g', "github", true);
            parser.add_option('l', "gitlab", true);
            let trimmed = args.get(1..).unwrap().to_vec();
            let result = parser.parse(trimmed.clone(), 0);
            
            for (k, _v) in result.iter() {
                provider = k.to_string();
            }

            let mut args_git: Vec<String> = Vec::new();
            args_git.push("clone".to_string());
            let mut new_args = remove_flags(args.get(2..).unwrap().to_vec());

            new_args[0] = format!("http://www.{}.com/{}.git", provider, new_args[0]);
            args_git.append(&mut new_args);

            call_git(args_git);
        }

        "go" => {
            /*
                * The go command - a shorthand for (add ., commit -m, push)
                * Very simple - just use hgit go, then all other args get appended.
            */
            call_str("add .");
            println!("{}", format!("commit -m \"{}\"", &args.get(2..).unwrap().to_vec().join(" ")));
            call_str(format!("commit -m \"{}\"", &args.get(2..).unwrap().to_vec().join(" ")));
            call_str("push");
        }

        // by default, just call git, but allow flags like --version
        _ => {
            let mut parser = cmd::start(None);
            parser.add_callback('v', "version", version_callback, true);
            let trimmed = args.get(1..).unwrap().to_vec();
            let result = parser.parse(trimmed.clone(), 0);
            if result.is_empty() {
                call_git(trimmed.clone());
            }
        }
    }

}
