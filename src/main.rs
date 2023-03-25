use std::env; // allows you to pass arguments to the commands line
use std::process; // provides abort and exit for terminating current process
use minigrep::Config; // brings project logic from lib.rs and Config struct types into scope

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);  // dbg! returns reference value

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);

    });

    if let Err(e) = minigrep::run(config) {
        print!("Application error: {e}");
        process::exit(1);
    }

}
