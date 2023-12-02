use std::env;

fn main() {
    my_solution();
}

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
