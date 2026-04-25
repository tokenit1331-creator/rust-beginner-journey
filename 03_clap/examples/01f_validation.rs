// ============================================================
// 🟡 ПРОВЕРКА ВВОДА: ValueEnum, range, кастомный парсер
// ============================================================
//
// clap умеет проверять что ввёл пользователь ДО того
// как твой код начнёт работать.
//
// ============================================================

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone)]
enum Network {
    Mainnet,
    Testnet,
    Devnet,
}

#[derive(Parser)]
struct Args {
    /// Адрес (должен начинаться с 0x)
    #[arg(value_parser = validate_address)]
    address: String,

    /// Сеть (mainnet, testnet, devnet)
    #[arg(short, long, default_value = "mainnet")]
    network: Network,

    /// Подтверждения (1..=100)
    #[arg(short, long, default_value_t = 12, value_parser = clap::value_parser!(u32).range(1..=100))]
    confirmations: u32,
}

fn validate_address(s: &str) -> Result<String, String> {
    if !s.starts_with("0x") {
        return Err(format!("'{}' — должен начинаться с 0x", s));
    }
    if s.len() < 3 {
        return Err("Адрес слишком короткий".to_string());
    }
    Ok(s.to_string())
}

fn main() {
    let args = Args::parse();
    println!("Адрес: {}", args.address);
    println!("Сеть: {:?}", args.network);
    println!("Подтверждений: {}", args.confirmations);
}

// --- ЗАПУСК ---
// cargo run --example 01f_validation -- 0xabc123 -n mainnet
// cargo run --example 01f_validation -- abc123     ← ошибка: не 0x
// cargo run --example 01f_validation -- 0xabc -n invalid  ← ошибка
