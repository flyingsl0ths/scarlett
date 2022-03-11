use std::env;
use std::process;

mod interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    let provided_script_name = args.len() == 2usize;

    if !provided_script_name {
        println!("Usage: scarlett <script>")
    }

    if let Err(msg) = interpreter::run(&args[1usize]) {
        println!("{}", msg);
        process::exit(1);
    }
}
