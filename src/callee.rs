use std::process::Command;


pub fn call_git(args: Vec<String>) {
    let _ = Command::new("git")
        .args(args)
        .status()
        .expect("Failed to execute command");

}

pub fn call_str<T: ToString>(cmd: T) {
    let cmd_final: String = cmd.to_string();
    call_git(cmd_final.split_whitespace()
    .map(|s| s.to_string())
    .collect());
}

pub fn version_callback(_:Option<String>) -> String {
    return format!("{}, hgit version {}", String::from_utf8(Command::new("git")
    .args(vec!["--version"])
    .output()
    .expect("Failed to execute command").stdout).unwrap().replace("\n", ""), env!("CARGO_PKG_VERSION"));
}

pub fn remove_flags(args: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    for arg in args {
        if arg.chars().nth(0).unwrap() != '-' {
            res.push(arg);
        }
    }

    res
}