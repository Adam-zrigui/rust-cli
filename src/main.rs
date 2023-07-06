use std::{env ,  process ,fs, error};
fn main() {

let config = Config::new(env::args()).unwrap_or_else(| err| {
    eprintln!("problem parsing arguments: {}", err);
    process::exit(1);
});
    println!("LOOKING FOR {}", config.q);
    println!("in file {}", config.filename);
if let Err(e) =run(config) {
    eprintln!("app error: {}", e);
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
     case_sensitive : bool,
}
impl Config {
     fn new( mut args : env::Args) -> Result<Config, &'static str> {
    args.next();
         let q = match args.next() {
             Some(args) => args,
             None => return Err("didnt get a query string")
     };

         let filename = match args.next() {
             Some(args) => args,
             None => return Err("didnt get a query string")
         };
         let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { q, filename , case_sensitive})
    }
}

pub fn search<'a>(q : &str , content: &'a str ) -> Vec<&'a str>{
content.lines().filter(|line| line.contains(q)).collect()
}