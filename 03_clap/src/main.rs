// ============================================================================
// КРИПТО-КОШЕЛЁК — ТОЧКА ВХОДА
// ============================================================================
//
// 💡 Запускай примеры по одному:
//
// Пример 1: Приветствие с базовыми аргументами
//   cargo run --example 01_hello_crypto -- 0x123abc
//
// Пример 2: Проверка баланса с валидацией
//   cargo run --example 02_check_balance -- 0x1234567890abcdef1234567890abcdef12345678
//
// Пример 3: Субкоманды и конфликты
//   cargo run --example 03_send_transaction -- send --to 0xabc --amount 0.5
//
// Пример 4: Полный крипто-кошелёк (env, count, кастомный help)
//   cargo run --example 04_crypto_wallet -- balance --address 0xabc
//
// Задание: Создай свой крипто-портфель!
//   cargo run --example exercise -- --help
//
// ============================================================================

fn main() {
    println!("🪙 Crypto CLI Урок — clap");
    println!("━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("📂 Смотреть примеры: examples/");
    println!("📖 Читать README:    README.md");
    println!();
    println!("📋 Доступные примеры:");
    println!("   01_hello_crypto     — Первый CLI");
    println!("   02_check_balance    — Валидация и ValueEnum");
    println!("   03_send_transaction — Субкоманды и конфликты");
    println!("   04_crypto_wallet    — Все фичи (env, count, help)");
    println!("   exercise            — Твоё задание!");
    println!();
    println!("🚀 Запусти: cargo run --example 01_hello_crypto -- --help");
}
