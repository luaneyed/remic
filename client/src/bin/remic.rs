extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Remic CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Sangguk Lee <sangguk258@gmail.com>")
        .about("Remic Client CLI")
        .subcommand(
            SubCommand::with_name("get")
                .about("get value by key")
                .arg(Arg::with_name("key").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("set value to key")
                .arg(Arg::with_name("key").index(1).required(true))
                .arg(Arg::with_name("val").index(2).required(true)),
        )
        .get_matches();

    let result = match matches.subcommand() {
        ("get", Some(args)) => remic::get(args.value_of("key").unwrap()).unwrap(),
        ("set", Some(args)) => {
            remic::set(args.value_of("key").unwrap(), args.value_of("val").unwrap()).unwrap()
        }
        (&_, _) => panic!("Not supporting command"),
    };
    println!("{}", result);
}
