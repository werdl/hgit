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
    callback: Option<fn(String) -> String>,
}

fn command(input: &str) -> Option<(String, String)> {
    let mut parts = input.splitn(2, '=');
    if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
        Some((k.to_string(), v.to_string()))
    } else {
        None
    }
}

impl Parser {
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

    pub fn parse(&mut self, args: Vec<String>, skip: usize) -> HashMap<String, String> {
        let mut to_ret = HashMap::new();
        let mut cur_flag = String::new();
    
        let mut args_iter = args.into_iter().skip(1 + skip); // Skip the first argument (program name)
    
        while let Some(arg) = args_iter.next() {
            if arg.starts_with('-') {
                match arg.chars().nth(1).unwrap_or('-') {
                    '-' => {
                        cur_flag = arg.get(2..).unwrap_or("").to_string();
                    }
                    x => {
                        cur_flag = x.to_string();
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

        let mut to_rem: Vec<String> = Vec::new();

        for (k, _v) in &to_ret {
            if !self.contains(&k) {
                match self.callback {
                    Some(x) => {
                        if k.contains("-") {
                            println!("{}", (x)(k.clone()));
                            exit(-1)
                        }
                    },
                    None => {
                        to_rem.push(k.to_string());
                    }
                };
            }
        }

        for rem in to_rem {
            to_ret.remove(&rem);
        }

        to_ret.retain(|key, _| !key.is_empty());
    
        to_ret
    }
    
    
      
}

pub fn start(wrong_arg_callback: Option<fn(String) -> String>) -> Parser {
    Parser {
        commands: Vec::new(),
        callback: wrong_arg_callback,
    }
}
