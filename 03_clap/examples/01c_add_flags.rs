// ============================================================
// 🟢 ДОБАВЛЯЕМ ФЛАГИ: short, long, bool
// ============================================================
//
// Флаг — аргумент, который либо есть, либо нет.
// Пример: --verbose, -v, --formal, -f
//
// ============================================================

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Твоё имя (обязательный)
    name: String,

    /// Подробный вывод (флаг)
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Официальное приветствие (флаг)
    #[arg(short = 'f', long = "formal")]
    formal: bool,
}

fn main() {
    let args = Args::parse();

    let greeting = if args.formal { "Здравствуйте" } else { "Привет" };

    if args.verbose {
        println!("[INFO] Программа запущена");
    }

    println!("{}, {}!", greeting, args.name);
}

// --- ЗАПУСК ---
// cargo run --example 01c_add_flags -- Артём
// cargo run --example 01c_add_flags -- Артём --formal
// cargo run --example 01c_add_flags -- Артём -v -f
