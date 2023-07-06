use std::{env ,  process ,fs, error};
fn main() {
    let args : Vec<String> = env::args().collect();
let config = Config::new(&args).unwrap_or_else(| err| {
    println!("problem parsing arguments: {}", err);
    process::exit(1);
});
    println!("LOOKING FOR {}", config.q);
    println!("in file {}", config.filename);
if let Err(e) =run(config) {
    println!("app error: {}", e);
    process::exit(1);
};
}
 fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let content = fs::read_to_string(config.filename)?;

    println!("with content {}", content);
    Ok(())
}
 struct  Config {
     q : String,
     filename : String,
}
impl Config {
     fn new(args : &[String]) -> Result<Config, &str> {
        if args.len() <= 3 {
            return Err("not enough arguments")
        }
        let q = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { q, filename })
    }
}
