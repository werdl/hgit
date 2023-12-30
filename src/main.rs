mod callee;
mod cmd;

extern crate rand;
extern crate colored;


use std::env;
use std::process::exit;
use std::process::Command;
use std::collections::HashMap;
use colored::ColoredString;
use rand::Rng;

use callee::*;

use colored::Colorize;
use colored::customcolors::CustomColor;

fn bad_arg(arg: String) -> String {
    return format!("Bad arg - {}", arg);
}

fn random<T: ToString>(command: T) -> ColoredString {
    return command.to_string().custom_color(CustomColor::new(rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen_range(0..=255), rand::thread_rng().gen_range(0..=255)))
}

fn call<T: ToString>(command: T) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command.to_string())
        .output()
        .expect("Failed to execute");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_commit_hashes() -> Vec<String> {
    // Run git command to get commit hashes on the default branch
    let output = Command::new("sh")
        .arg("-c")
        .arg("git log --format=%H --reverse $(git rev-list --max-parents=0 HEAD)..HEAD")
        .output()
        .expect("Failed to execute command");

    // Convert the output to a string and split by lines
    let output_str = String::from_utf8_lossy(&output.stdout);
    let commit_hashes: Vec<String> = output_str
        .lines()
        .map(|s| s.trim().to_string().get(..6).unwrap().to_string())
        .collect();

    commit_hashes
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let zero = match args.get(1) {
        None => {
            println!(
                "hgit version {}, licensed under the Apache License v2.0",
                env!("CARGO_PKG_VERSION")
            );
            exit(-1);
        }
        Some(x) => x,
    };
    match zero.as_str() {
        "info" => {
            /*
             * The info command - for getting commit info
             */
            let hashes = get_commit_hashes();

            for hash in hashes {
                let output = Command::new("git")
                    .args(&[
                        "log",
                        "-n1",
                        "--pretty=format:%cd %an",
                        "--date=local",
                        hash.as_str(),
                    ])
                    .output()
                    .expect("Failed to execute git command");

                let commit_info = String::from_utf8_lossy(&output.stdout);
                let (date, author) = (
                    commit_info
                        .split_whitespace()
                        .rev()
                        .skip(1)
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .rev()
                        .collect::<Vec<&str>>()
                        .join(" "),
                    commit_info.split_whitespace().last().unwrap_or("anonymous"),
                );

                let command = call(format!(
                    "git show --shortstat {} | tail -1 | awk '{{print $4, $6}}'",
                    hash
                ));
                let lines: Vec<String> = command
                    .split(' ')
                    .map(|x| x.replace("\n", ""))
                    .into_iter()
                    .collect();
                let name = call(format!("git log -n1 --pretty=format:%s {}", hash));
                println!(
                    "{} {} by {} ({}) +{} -{}",
                    date,
                    hash,
                    author,
                    name,
                    (if lines[0].trim() == "" { "0" } else { &lines[0] }).green(),
                    (if lines[1].trim() == "" { "0" } else { &lines[1] }).red(),
                );
            }
        }
        "data" => {
            /*
                * The data command
                * for displaying repo info, bitesize!
            */

            let default = call("git rev-parse --abbrev-ref HEAD");
            let temp = call("git branch");
            let mut temp2 = temp.split(" ").collect::<Vec<&str>>();
            temp2.reverse();
            let current = temp2.get(0).unwrap();

            let mut top_contrib: HashMap<String, i64> = HashMap::new();

            let raw_c = call(
                "git log --format='%aN' | sort | uniq -c | sort -rn"
            );

            let contributors = raw_c.trim().split("\n");

            for contrib in contributors.into_iter() {
                let clauses: Vec<&str> = contrib.split(" ").collect();
                top_contrib.insert(clauses[1].to_string(), clauses[0].parse().unwrap());
            }

            let loc = call(
                "git ls-files | grep -v -e '\\.md$' -e 'LICENSE$' | xargs wc -l | tail -1 | grep -o '[0-9]\\+'"
            );


            let name = call("basename $(git rev-parse --show-toplevel)");

            let raw_lf = call("git diff --name-only");

            let loose_files = raw_lf.trim().split("\n").collect::<Vec<&str>>();

            println!("{:?}", loose_files);

            println!("{} (default branch {}, current {}), with {} lines. {} files have uncommited changes ({}) \nContributors:",
                random(name.trim()),
                default.trim().blue(),
                current.trim().yellow(),
                loc.trim().green(),
                loose_files.len().to_string().red(),
                loose_files.join(", ").cyan()
            );
            let mut i = 0;
            for (person, commits) in top_contrib {
                i+=1;
                println!("\t{}. {} - {} commits", i, random(person), commits.to_string().green());
            }
        }
        
        "update" => {
            /*
             * the update command - stashes your changes, pulls and then pops your changes back.
             */

            call("git stash"); // stash local changes
            call("git pull"); // pull from origin
            let res = Command::new("git")
                .arg("stash")
                .arg("pop")
                .output();
            match res {
                Ok(x) => {
                    match x.status.code().unwrap() {
                        128 => {
                            println!("Merge conflict detected - please manually resolve.");
                            call("git stash apply stash@{0}");
                        },
                        _ => {
                            println!("Updates merged from origin");
                        }
                    }
                },
                Err(x) => {
                    println!("Errored out with {}", x);
                    exit(-1);
                }
            }
        }

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

        "template" => {
            /*
             * create a new repository, from template
             * templates are pulled from any repository
             */
            let mut provider = "github".to_string();

            let mut parser = cmd::start(Some(bad_arg));
            parser.add_option('g', "github", true);
            parser.add_option('l', "gitlab", true);
            parser.add_option('n', "name", false);
            let trimmed = args.get(1..).unwrap().to_vec();
            let result = parser.parse(trimmed.clone(), 0);

            if !result.contains_key("name") {
                println!("Please specify a name!");
                exit(-1);
            }

            for (k, _v) in result.iter() {
                if vec!["gitlab", "github"].contains(&k.as_str()) {
                    provider = k.to_string();
                }
            }

            let mut args_git: Vec<String> = Vec::new();
            args_git.push("clone".to_string());
            let mut new_args = remove_flags(args.get(2..).unwrap().to_vec());

            let name = result["name"].clone();

            new_args[0] = format!("http://www.{}.com/{}.git", provider, new_args[0]);
            args_git.append(&mut new_args);
            args_git.push(result["name"].clone()); // clone into given subdir

            println!("{:?}", args_git);
            call_git(args_git); // clone into a new directory

            Command::new("rm")
                .args(vec!["-rf", format!("./{}/.git", name).as_str()])
                .status()
                .expect("Failed to execute command");

            call_str(format!("init {0} --template={0}", name));
        }

        "go" => {
            /*
             * The go command - a shorthand for (add ., commit -m, push)
             * Very simple - just use hgit go, then all other args get appended.
             */
            call_str("add .");
            call_git(vec![
                "commit".to_string(),
                "-m".to_string(),
                format!("{}", &args.get(2..).unwrap().to_vec().join(" ")),
            ]);
            call_str("push");
        }

        "version" => {
            /*
             * version command
             * gives git and hgit version
             */
            println!("{}", version_callback(None));
        }

        // by default, just call git
        _ => {
            let trimmed = args.get(1..).unwrap().to_vec();
            call_git(trimmed);
        }
    }
}
