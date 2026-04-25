// ============================================
// 🟡 Шаг 2: JSON + Serde (сериализация структур)
// ============================================
//
// КОНТЕКСТ: Сохраняем конфиг крипто-кошелька в JSON.
//
// ЧТО ТЫ УЗНАЕШЬ:
// - #[derive(Serialize, Deserialize)] — магия Serde
// - serde_json::to_string_pretty() — структура → JSON
// - serde_json::from_str() — JSON → структура
// - Вложенные структуры в JSON
// - Как работает ? с Serde

use serde::{Deserialize, Serialize};
use std::fs;

// ===========================================
// 1. ОПРЕДЕЛЯЕМ СТРУКТУРЫ
// ===========================================
//
// #[derive(Serialize, Deserialize)] — это макросы из крейта serde.
// Они автоматически генерируют код для:
// - Serialize: превратить структуру в JSON (и обратно)
// - Deserialize: превратить JSON обратно в структуру
//
// 💡 ADVICE: Serde — стандарт де-факто для сериализации в Rust.
// Он поддерживает JSON, YAML, TOML, BSON, MessagePack и сотни других форматов.
// Один раз написал #[derive] — работает везде.
//
// ПОЧЕМУ: Serde генерирует код на этапе компиляции.
// Это значит: если ты опечатался в названии поля —
// программа даже не скомпилируется. Безопасность на уровне типов.

/// Конфиг кошелька
///
/// #[serde(rename_all = "snake_case")] — Serde будет
/// записывать поля в JSON в snake_case (как принято в JSON).
/// Без этого поля были бы в camelCase (как в Rust).
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct WalletConfig {
    /// Адрес кошелька (0x...)
    address: String,
    /// Сеть (mainnet / testnet / devnet)
    network: String,
    /// Баланс в токенах
    balance: f64,
    /// Список токенов (вложенная структура)
    tokens: Vec<TokenInfo>,
    /// Публичный ключ (hex-строка)
    public_key: String,
}

/// Информация о токене
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct TokenInfo {
    symbol: String,
    name: String,
    balance: f64,
    decimals: u8,
}

fn main() -> std::io::Result<()> {
    // ===========================================
    // 2. СОЗДАЁМ ОБЪЕКТ
    // ===========================================
    let config = WalletConfig {
        address: "0x742d35Cc6634C0532925a3b844Bc9e7595f3bD1f".to_string(),
        network: "mainnet".to_string(),
        balance: 42.5,
        tokens: vec![
            TokenInfo {
                symbol: "ETH".to_string(),
                name: "Ethereum".to_string(),
                balance: 1.5,
                decimals: 18,
            },
            TokenInfo {
                symbol: "USDT".to_string(),
                name: "Tether USD".to_string(),
                balance: 1000.0,
                decimals: 6,
            },
            TokenInfo {
                symbol: "SOL".to_string(),
                name: "Solana".to_string(),
                balance: 15.0,
                decimals: 9,
            },
        ],
        public_key: "0x04a6b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7".to_string(),
    };

    // ===========================================
    // 3. СТРУКТУРА → JSON
    // ===========================================
    //
    // serde_json::to_string_pretty() — превращает структуру
    // в JSON-строку с отступами (pretty-print).
    //
    // 💡 ADVICE: Используй pretty для конфигов (читается человеком)
    // и to_string() для передачи по сети (меньше размер).
    //
    // ПОЧЕМУ: ? здесь работает, потому что serde_json::to_string_pretty
    // возвращает Result<String, serde_json::Error>, а main() — Result<(), std::io::Error>.
    // Но у нас есть преобразование: serde_json::Error → std::io::Error?
    // На самом деле нет — мы используем ? с преобразованием...
    // Упрощённо: возвращаем String, а fs::write принимает &str.
    let json = serde_json::to_string_pretty(&config)
        .expect("Ошибка сериализации JSON");  // serde_json::Error -> паника

    println!("📄 JSON, который будет сохранён:");
    println!("{}", json);

    // Записываем JSON в файл
    fs::write("wallet_config.json", &json)?;
    println!("✅ Конфиг кошелька сохранён в wallet_config.json");

    // ===========================================
    // 4. JSON → СТРУКТУРА
    // ===========================================
    //
    // Читаем JSON из файла и превращаем обратно в структуру.
    //
    // serde_json::from_str() — читает JSON из строки и
    // восстанавливает структуру. Тип указывается после ::
    let saved_json = fs::read_to_string("wallet_config.json")?;
    let loaded: WalletConfig = serde_json::from_str(&saved_json)
        .expect("Ошибка десериализации JSON");

    println!();
    println!("📖 Загруженный конфиг:");
    println!("   Адрес:    {}", loaded.address);
    println!("   Сеть:     {}", loaded.network);
    println!("   Баланс:   {} ETH", loaded.balance);
    println!("   Токены:");
    for token in &loaded.tokens {
        println!("     - {} ({}): {} ", token.symbol, token.name, token.balance);
    }

    // ===========================================
    // 5. УБОРКА
    // ===========================================
    // Удаляем временный файл
    fs::remove_file("wallet_config.json")?;
    println!("🗑️ Временный файл удалён");

    println!();
    println!("🎉 Урок 2 завершён! Ты научился:");
    println!("   - Определять структуры с Serde");
    println!("   - Превращать структуру в JSON и обратно");
    println!("   - Сохранять/загружать конфиги из файла");

    Ok(())
}
