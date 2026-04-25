// ============================================================
// 🟢 ПОВТОРЕНИЕ: все подходы в одном файле
// ============================================================
//
// Собирает ВСЁ что ты выучил в один пример.
//
// ============================================================

use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Args {
    // 1. Позиционный аргумент
    name: String,

    // 2. Флаг (bool)
    #[arg(short, long)]
    verbose: bool,

    // 3. Опция с дефолтом
    #[arg(short, long, default_value_t = 1)]
    count: u32,

    // 4. Опциональный
    #[arg(short, long)]
    network: Option<String>,

    // 5. ValueEnum
    #[arg(short = 'm', long = "mode", default_value = "fast")]
    mode: Mode,

    // 6. Range
    #[arg(short, long, default_value_t = 5, value_parser = clap::value_parser!(u32).range(1..=10))]
    level: u32,
}

#[derive(ValueEnum, Clone)]
enum Mode { Fast, Normal, Slow }

fn main() {
    let args = Args::parse();

    println!("\n=== ТЫ ВЫУЧИЛ: ===");
    println!("1. Позиционный: name = {}", args.name);
    println!("2. Флаг: verbose = {}", args.verbose);
    println!("3. Дефолт: count = {}", args.count);
    println!("4. Option: network = {:?}", args.network);
    println!("5. ValueEnum: mode = {:?}", args.mode);
    println!("6. Range: level = {}", args.level);
    println!("\n✅ Всё работает!");
}

// --- ЗАПУСК ---
// cargo run --example 01g_review -- Артём -v --count 3 --network mainnet --mode slow --level 7
