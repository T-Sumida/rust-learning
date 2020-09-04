extern crate rust_learning;

use std::env;
use std::process;

use rust_learning::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数解析時に問題
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rust_learning::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
