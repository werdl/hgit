use std::collections::HashMap;
use std::process::exit;

pub struct Command<'a> {
    long: &'a str,
    short: char,
    desc: &'a str,
}

pub struct Parser {
    commands: Vec<Command<'static>>,
    desc: &'static str,
    name: &'static str,
    callback: fn() -> String,
}

fn command(input: &str) -> Option<(String, String)> {
    let splitted: Vec<&str> = input.splitn(2, '=').collect();
    if splitted.len() == 2 {
        Some((splitted[0].to_string(), splitted[1].to_string()))
    } else {
        None
    }
}

impl Parser {
    pub fn add(&mut self, short: char, long: &'static str, desc: &'static str) {
        self.commands.push(Command{
            long,
            short,
            desc,
        });
    }

    pub fn long(&mut self, input: String) -> String {
        for cmd in &self.commands {
            if cmd.short.to_string() == input {
                return cmd.long.to_string();
            }
        }
        input
    } 

    fn contains(&self, command: &str) -> bool {
        for cmd in &self.commands {
            if cmd.short.to_string() == command || cmd.long == command {
                return true;
            }
        }
        false
    }

    pub fn parse(&mut self, args: std::env::Args) -> HashMap<String, String> {
        let mut to_ret = HashMap::new();
        let mut cur_flag = String::new();

        let mut args_iter = args.skip(1); // skip the first argument (program name)

        while let Some(arg) = args_iter.next() {
            if arg.starts_with('-') {
                match arg.chars().nth(1).unwrap() {
                    '-' => {
                        cur_flag = arg.get(2..).unwrap_or("").to_string();
                    }
                    x => {
                        cur_flag = x.to_string();
                    }
                }

                if let Some((k, v)) = command(arg.get(2..).unwrap_or("")) {
                    to_ret.insert(self.long(k), v);
                }
            } else {
                let flag = cur_flag.clone();
                if !flag.is_empty() {
                    to_ret.insert(self.long(flag), arg);
                }
            }

            if !self.contains(&cur_flag) {
                println!("{}", (self.callback)());
                exit(-1);
            }
        }

        to_ret
    }
}

pub fn start(name: &'static str, desc: &'static str, wrong_arg_callback: fn() -> String) -> Parser {
    Parser {
        name,
        desc,
        commands: Vec::new(),
        callback: wrong_arg_callback,
    }
}
