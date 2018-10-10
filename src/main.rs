use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        //panicと違いRUST_BAKTRACEなど余計な物を出さずにプログラミングを終了できる
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application Error: {}", e);

        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            //ユーザーが値を入力するような提供のためのコードはnewの条件をpanicにするのではなくErrにするのが望ましい。開発者相手のlib開発にはpanicが望ましい。
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
