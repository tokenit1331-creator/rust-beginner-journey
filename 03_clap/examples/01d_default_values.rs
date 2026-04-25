// ============================================================
// 🟢 ЗНАЧЕНИЯ ПО УМОЛЧАНИЮ: default_value_t
// ============================================================
//
// Если пользователь не указал аргумент — используем дефолт.
//
// ============================================================

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Сумма счёта (обязательный)
    sum: f64,

    /// Количество человек (по умолчанию 1)
    #[arg(short = 'p', long = "persons", default_value_t = 1)]
    persons: u32,

    /// Процент чаевых (по умолчанию 10)
    #[arg(short = 't', long = "tip", default_value_t = 10)]
    tip_percent: u32,

    /// Валюта (по умолчанию USD)
    #[arg(short = 'c', long = "currency", default_value_t = String::from("USD"))]
    currency: String,
}

fn main() {
    let args = Args::parse();

    let tip = args.sum * args.tip_percent as f64 / 100.0;
    let total = args.sum + tip;
    let per_person = total / args.persons as f64;

    println!("=== СЧЁТ ===");
    println!("Сумма: {:.2} {}", args.sum, args.currency);
    println!("Чаевые: {:.2} {}", tip, args.currency);
    println!("С каждого: {:.2} {}", per_person, args.currency);
}

// --- ЗАПУСК ---
// cargo run --example 01d_default_values -- 1000
// cargo run --example 01d_default_values -- 5000 -p 4 -t 15 -c EUR
