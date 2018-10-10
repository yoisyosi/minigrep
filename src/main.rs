extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        //panicと違いRUST_BAKTRACEなど余計な物を出さずにプログラミングを終了できる
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {}", e);

        process::exit(1);
    }
}
