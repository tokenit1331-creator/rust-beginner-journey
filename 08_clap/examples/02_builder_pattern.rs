// Второй способ: Builder Pattern (без макросов)
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("myapp")
        .version("1.0")
        .author("Rust Mentor")
        .about("Учимся использовать Builder Pattern")
        .arg(
            Arg::new("name")
                .help("Ваше имя")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Подробный вывод"),
        )
        .get_matches();

    let name = matches.get_one::<String>("name").unwrap();
    let verbose = matches.get_flag("verbose");

    if verbose {
        println!("[INFO] Привет, {}!", name);
    } else {
        println!("Привет, {}!", name);
    }
}
