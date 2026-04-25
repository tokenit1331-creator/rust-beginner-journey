// ============================================================
// 🟢 ОПЦИОНАЛЬНЫЕ АРГУМЕНТЫ: Option<T>
// ============================================================
//
// Option — когда аргумент может быть, а может и НЕ быть.
// None = не указано. Some(...) = указано.
//
// ============================================================

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Адрес кошелька (обязательный)
    address: String,

    /// Сеть (опционально)
    #[arg(short = 'n', long = "network")]
    network: Option<String>,

    /// API-ключ (опционально)
    #[arg(short = 'k', long = "api-key")]
    api_key: Option<String>,

    /// Таймаут (по умолчанию 30)
    #[arg(short = 't', long = "timeout", default_value_t = 30)]
    timeout: u64,
}

fn main() {
    let args = Args::parse();

    println!("Адрес: {}", args.address);

    match &args.network {
        Some(net) => println!("Сеть: {}", net),
        None => println!("Сеть: не указана"),
    }

    match &args.api_key {
        Some(key) => println!("API-ключ: {}...", &key[..4]),
        None => println!("API-ключ: не указан"),
    }

    println!("Таймаут: {} сек", args.timeout);
}

// --- ЗАПУСК ---
// cargo run --example 01e_optional_values -- 0xABC
// cargo run --example 01e_optional_values -- 0xABC -n mainnet -k sk-12345678
