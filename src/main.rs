mod parser;

fn main() {
    let file_parser = parser::FileConfigParser::new('=');
    let config = parser::Config::new(file_parser, "config.cfg");

    match config {
        Ok(conf) => {
            if let Some(val) = conf.get("data") {
                println!("Value: {}", val);
            } else {
                println!("Key not found.");
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}
