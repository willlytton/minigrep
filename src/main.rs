// allows you to pass arguments to the commands line
use std::env::args;

fn main() {
   let args: Vec<String> = env::args().collect();
   dbg!(args);  // dbg! returns reference value 
}
