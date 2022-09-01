fn messenger(argument: &str) -> std::result::Result<String, String> {
    match argument {
        "salutare" => Ok("sal".to_string()),
        "hello" => Ok("buna ziua".to_string()),
        "error" => Err("error".to_string()),
        _ => Ok(String::from("default"))
    }
}

fn main() {
    let argument = std::env::args().nth(1).unwrap_or_else(|| {
        println!("must supply an integer argument");
        std::process::exit(1);
    });

    let result = messenger(&argument);
    match result {
        Ok(num) => println!("{}", num),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
