extern crate remic;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    remic::execute(&args[1..].join(" ")).unwrap();
}
