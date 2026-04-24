use clap::Parser;

/// Простая программа для демонстрации clap
#[derive(Parser)]
#[command(name = "greet")]
#[command(version = "1.0")]
#[command(about = "Говорит привет пользователю")]
struct Cli {
    /// Имя пользователя
    name: String,

    /// Сколько раз поздороваться (по умолчанию 1)
    #[arg(short = 'n', long = "count", default_value_t = 1)]
    count: u32,

    /// Сделать приветствие официальным
    #[arg(short = 'f', long = "formal")]
    formal: bool,
}

fn main() {
    let cli = Cli::parse();

    let greeting = if cli.formal {
        format!("Здравствуйте, {}!", cli.name)
    } else {
        format!("Привет, {}!", cli.name)
    };

    for _ in 0..cli.count {
        println!("{}", greeting);
    }
}
