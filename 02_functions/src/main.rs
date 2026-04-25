// ============================================================
// 📋 НАВИГАТОР ПО ПРИМЕРАМ
// ============================================================
//
// Этот файл — просто "меню". Он показывает, какие примеры есть
// и как их запускать. Сам код — в папке examples/.
//
// ДЛЯ НОВИЧКА: Не читай этот файл как урок.
// Открой examples/01_basic.rs — там начинается обучение.
// ============================================================

fn main() {
    println!("=== Урок: Функции и управление в Rust ===\n");
    println!("📂 Папка: 02_functions\n");
    println!("📖 Читай в таком порядке:\n");
    println!("  1️⃣  examples/01_basic.rs");
    println!("     → Базовые функции: параметры, return, ссылки");
    println!("     → Запуск: cargo run --example 01_basic\n");
    println!("  2️⃣  examples/02_crypto_trading.rs");
    println!("     → Реальная крипто-биржа: колбэки, замыкания");
    println!("     → Запуск: cargo run --example 02_crypto_trading\n");
    println!("  3️⃣  examples/03_advanced.rs");
    println!("     → Продвинутые: map/filter/fold, обобщения");
    println!("     → Запуск: cargo run --example 03_advanced\n");
    println!("  4️⃣  exercises/exercise.rs");
    println!("     → 5 заданий с подсказками — проверь себя!");
    println!("     → Запуск: cargo run --example exercise\n");
    println!("💡 ADVICE.md — 12 советов от опытного разработчика\n");
    println!("🚀 Начни с: cargo run --example 01_basic");
}
