use std::collections::HashMap;
use std::process::exit;

#[derive(Copy, Clone)]
pub struct Command<'a> {
    long: &'a str,
    short: char,
    callback: Option<fn(Option<String>) -> String>,
    is_flag: bool,
}

pub struct Parser {
    commands: Vec<Command<'static>>,
    desc: &'static str,
    name: &'static str,
    callback: fn() -> String,
}

fn command(input: &str) -> Option<(String, String)> {
    let mut parts = input.splitn(2, '=');
    if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
        Some((k.to_string(), v.to_string()))
    } else {
        None
    }
}


/// Used if an option is a flag
pub fn does_nothing(_: Option<String>) -> String {
    "".to_string()
}

impl Parser {
    fn has_func(&self, inp: Option<fn(Option<String>) -> String>) -> bool {
        inp.is_some()
    }

    fn check_for_clash(&self, short: char, long: &str) -> bool {
        self.contains(&short.to_string()) || self.contains(long) || short.to_string() == long.to_string()
    }

    pub fn add_option(&mut self, short: char, long: &'static str, is_flag: bool) -> Option<bool> {
        if self.check_for_clash(short, long) {
            return None;
        }
        self.commands.push(Command {
            long,
            short,
            callback: None,
            is_flag,
        });
        Some(true)
    }

    pub fn add_callback(
        &mut self,
        short: char,
        long: &'static str,
        callback: fn(Option<String>) -> String,
        is_flag: bool,
    ) -> Option<bool> {
        if self.check_for_clash(short, long) {
            return None;
        }
        self.commands.push(Command {
            long,
            short,
            callback: Some(callback),
            is_flag,
        });
        Some(true)
    }

    pub fn long(&self, input: &str) -> String {
        self.commands
            .iter()
            .find(|cmd| cmd.short.to_string() == input)
            .map_or_else(|| input.to_string(), |cmd| cmd.long.to_string())
    }

    fn contains(&self, command: &str) -> bool {
        self.commands
            .iter()
            .any(|cmd| cmd.short.to_string() == command || cmd.long == command)
    }

    fn command(&self, name: &str) -> Option<Command> {
        self.commands
            .iter()
            .find(|cmd| cmd.short.to_string() == name || cmd.long == name)
            .cloned()
    }

    fn is_flag(&self, name: &str) -> bool {
        self.command(name).map_or(false, |cmd| cmd.is_flag)
    }

    pub fn parse(&mut self, args: std::env::Args) -> HashMap<String, String> {
        let mut to_ret = HashMap::new();
        let mut cur_flag = String::new();
        let mut single_double = "";
    
        let mut args_iter = args.skip(1); // Skip the first argument (program name)
    
        while let Some(arg) = args_iter.next() {
            if arg.starts_with('-') {
                match arg.chars().nth(1).unwrap_or('-') {
                    '-' => {
                        cur_flag = arg.get(2..).unwrap_or("").to_string();
                        single_double = "double";
                    }
                    x => {
                        cur_flag = x.to_string();
                        single_double = "single";
                    }
                }
    
                if arg.contains('=') {
                    let parts: Vec<&str> = arg.splitn(2, '=').collect();
                    let dash_count = arg.matches('-').count();
                    let counts = if dash_count == 2 {
                        parts[0].trim_start_matches('-').to_string()
                    } else {
                        cur_flag.clone()
                    };
                
                    if !self.is_flag(&counts) {
                        to_ret.insert(self.long(&counts), parts[1].to_string());
                        if let Some(callback) = self.command(&counts).and_then(|cmd| cmd.callback) {
                            println!("{}", callback(Some(parts[1].to_string())));
                        }
                    } else {
                        to_ret.insert(self.long(&counts), "true".to_string());
                        if let Some(callback) = self.command(&counts).and_then(|cmd| cmd.callback) {
                            println!("{}", callback(None));
                        }
                    }
                    cur_flag.clear();
                } else {
                    if self.is_flag(&cur_flag) {
                        if let Some(callback) = self.command(&cur_flag).and_then(|cmd| cmd.callback) {
                            if self.is_flag(&cur_flag) {
                                println!("{}", callback(None));
                            }
                        }
                        to_ret.insert(self.long(&cur_flag), true.to_string());
                    }
                }
                
            } else {
                let k = cur_flag.clone();
                let (key, value) = if let Some((k, v)) = command(&arg) {
                    (k, v)
                } else {
                    (k.clone(), arg.clone())
                };


    
                to_ret.insert(self.long(&key), value.clone());
                if let Some(callback) = self.command(&key).and_then(|cmd| cmd.callback) {
                    if !self.is_flag(&key) {
                        println!("{}", callback(Some(value)));
                    }
                }
    
                cur_flag.clear();
            }
        }

        for (k, v) in &to_ret {
            if !self.contains(k) {
                println!("{}", (self.callback)());
                exit(-1)
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
