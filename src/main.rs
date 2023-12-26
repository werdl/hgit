mod callee;
mod cmd;

use std::env;

fn cb() -> String {
    return "welp we failed".to_string()
}

fn main() {
    let mut parser = cmd::start("test", "test thingy", cb);
    parser.add('c', "colour", "Colour of choice");
    println!("{:?}", parser.parse(env::args()));
}
