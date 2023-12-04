// POSIX-compatible programs should exit with a value of 0 to indicate success
// and any value between 1 and 255 to indicate an error.
fn main() {
    std::process::exit(1);
    // abort() function can also terminate the process with a nonzero error
    // code, see docs for details.
    // std::process::abort();
}
