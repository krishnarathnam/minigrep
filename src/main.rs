use minigrep::search;
use std::env;
use std::fs;
use std::io::Error;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    println!("{}", config.query);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Error> {
    let file_result = fs::File::open(config.file_path);
    let mut file = match file_result {
        Ok(file) => file,
        Err(e) => panic!("Error cannot read file in this path: {e:?}"),
    };

    let content = read_the_file(&mut file).unwrap_or_else(|err| {
        println!("Problem Reading content of file: {err}");
        std::process::exit(1);
    });

    for line in search(&config.query, &content) {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn read_the_file(content_file: &mut fs::File) -> Result<String, Error> {
    let mut content = String::new();
    match content_file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}
