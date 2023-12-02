use std::env;

fn main() {
    my_solution();
}

fn my_solution() {
    let mut args = env::args().skip(1);

    let mut res = String::new();

    let mut trailing_newline = true;
    match args.nth(0) {
        Some(s) => {
            if s == "-n" {
                res.insert_str(0, args.next().unwrap().as_str());
                trailing_newline = false;
            } else {
                res.insert_str(0, s.as_str());
            }
        }
        None => {}
    };

    for arg in args {
        res.insert(res.len(), ' ');
        res.insert_str(res.len(), arg.as_str());
    }

    if trailing_newline {
        res.insert(res.len(), '\n');
    }

    print!("{}", res);
}
