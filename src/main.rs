mod callee;
mod cmd;

use std::env;

fn cb() -> String {
    return "welp we failed".to_string()
}

fn cb2(arg: Option<String>) -> String {
    match arg {
        None => {
            return "oh, no fun :-(".to_string();
        },
        Some(x) => {
            return format!("we got {}", x);
        }
    }
}

fn main() {
    let mut parser = cmd::start("test", "test thingy", cb);
    parser.add_option('c', "colour", false);
    parser.add_option('p', "present", true);
    parser.add_callback('q', "quash", cb2, false);
    println!("{:?}", parser.parse(env::args()));
}
