use std::env;

use clap::{App, Arg};

fn main() {
    let matches = App::new("echo")
        .version("0.1.0")
        .author("Pengyu Li <lipengyux@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print trailing newline")
                .takes_value(false),
        )
        .get_matches();

    println!("{:#?}", matches);
}

#[allow(dead_code)]
fn my_solution() {
    let mut args = env::args().skip(1).peekable();

    let mut res = String::new();

    let trailing_newline = match args.peek() {
        Some(s) => {
            if s == "-n" {
                args.next();
                false
            } else {
                true
            }
        }
        None => true,
    };

    for arg in args {
        res.push_str(arg.as_str());
        res.push(' ');
    }

    // removes the last ' '
    res.pop();

    if trailing_newline {
        res.push('\n');
    }

    print!("{}", res);
}
