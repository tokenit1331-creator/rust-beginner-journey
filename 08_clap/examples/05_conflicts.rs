// ============================================================
// ПРИМЕР 5: ЗАВИСИМОСТИ И КОНФЛИКТЫ МЕЖДУ АРГУМЕНТАМИ
// ============================================================
// clap позволяет задавать логические связи:
//   - conflicts_with: не могут быть указаны вместе
//   - requires: один аргумент требует другой
//   - required_unless_present: обязателен, если не указан другой
//   - group: объединение аргументов в группу
// ============================================================

use clap::{Parser, ArgGroup};

/// Демонстрация логических связей между аргументами
#[derive(Parser)]
#[command(name = "shipment")]
#[command(group(
    ArgGroup::new("transport")
        .required(true)
        .args(["ship", "plane", "truck"]),
))]
struct Args {
    // ============================================================
    // ГРУППА: нужно выбрать ровно один способ доставки
    // ============================================================

    /// Отправить кораблём
    #[arg(short, long)]
    ship: bool,

    /// Отправить самолётом
    #[arg(short, long)]
    plane: bool,

    /// Отправить грузовиком (конфликтует с --plane)
    #[arg(short, long, conflicts_with = "plane")]
    truck: bool,

    // ============================================================
    // ЗАВИСИМОСТИ: --insured требует --value
    // ============================================================

    /// Застраховать груз (требует --value)
    #[arg(short, long, requires = "value")]
    insured: bool,

    /// Стоимость груза для страховки
    #[arg(short, long)]
    value: Option<f64>,

    // ============================================================
    // УСЛОВНАЯ ОБЯЗАТЕЛЬНОСТЬ
    // ============================================================

    /// Вес груза в кг (обязателен, если не указан --volume)
    #[arg(short = 'w', long)]
    weight: Option<f64>,

    /// Объём груза в м³ (обязателен, если не указан --weight)
    #[arg(short, long, required_unless_present = "weight")]
    volume: Option<f64>,
}

fn main() {
    let args = Args::parse();

    let transport = if args.ship { "🚢 корабль" }
        else if args.plane { "✈️ самолёт" }
        else { "🚚 грузовик" };
    println!("📦 Способ доставки: {}", transport);

    if args.insured {
        println!("🛡️ Груз застрахован на ${}", args.value.unwrap());
    }

    if let Some(w) = args.weight {
        println!("⚖️ Вес: {} кг", w);
    }
    if let Some(v) = args.volume {
        println!("📐 Объём: {} м³", v);
    }
}

// Примеры запуска:
// cargo run --example 05_conflicts -- --ship          ✅
// cargo run --example 05_conflicts -- --ship --plane  ❌ (конфликт группы)
// cargo run --example 05_conflicts -- --plane --truck ❌ (conflicts_with)
// cargo run --example 05_conflicts -- --ship --insured --value 1000  ✅
// cargo run --example 05_conflicts -- --ship --insured  ❌ (нужен --value)
// cargo run --example 05_conflicts -- --ship --weight 5  ✅
// cargo run --example 05_conflicts -- --plane           ✅ (volume не обязателен, т.к. weight не указан...)
// cargo run --example 05_conflicts --                  ❌ (нужен transport)
