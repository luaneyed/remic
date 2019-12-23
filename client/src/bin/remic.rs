use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{}", remic::execute(&args[1..].join(" ")).unwrap());
}
