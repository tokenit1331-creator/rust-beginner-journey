// ============================================================
// ПРИМЕР 4: ВАЛИДАЦИЯ ЗНАЧЕНИЙ
// ============================================================
// clap позволяет проверять значения на разных уровнях:
//   1. Тип (u32, String, IpAddr — автоматически)
//   2. Диапазон (.range(1..=100))
//   3. Кастомный парсер (value_parser с fn или closure)
//   4. Регулярные выражения (через validator)
//   5. Возможные значения (value_enum / ValueEnum)
// ============================================================

use clap::{Parser, ValueEnum};

// ============================================================
// ValueEnum — перечисление допустимых значений
// clap сам подскажет в --help какие варианты возможны
// ============================================================
#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Mode {
    /// Быстрый режим
    Fast,
    /// Тщательный режим
    Slow,
    /// Режим по умолчанию
    Normal,
}

/// Демонстрация всех видов валидации
#[derive(Parser)]
#[command(name = "validate")]
struct Args {
    // ============================================================
    // ВАЛИДАЦИЯ 1: Встроенная — типобезопасность
    // ============================================================
    // clap сам проверяет, что число — это число, а не буква

    /// Возраст (автоматически проверяется, что это u32)
    #[arg(short, long)]
    age: u32,

    // ============================================================
    // ВАЛИДАЦИЯ 2: Диапазон значений
    // ============================================================
    // value_parser!(u32).range(1..=150) — только 1..150

    /// Вес в кг (диапазон 1..=300)
    #[arg(short, long, default_value_t = 70, value_parser = clap::value_parser!(u32).range(1..=300))]
    weight: u32,

    // ============================================================
    // ВАЛИДАЦИЯ 3: ValueEnum — строгий набор значений
    // ============================================================

    /// Режим работы (fast/slow/normal)
    #[arg(short, long, default_value_t = Mode::Normal)]
    mode: Mode,

    // ============================================================
    // ВАЛИДАЦИЯ 4: Кастомная функция-парсер
    // ============================================================

    /// IP-адрес (кастомный парсер)
    #[arg(short, long, value_parser = parse_ip)]
    ip: Option<String>,

    // ============================================================
    // ВАЛИДАЦИЯ 5: Регулярное выражение (через closure)
    // ============================================================

    /// Email (проверка через регулярку)
    #[arg(short, long, value_parser = validate_email)]
    email: Option<String>,
}

/// Кастомный парсер для IP-адреса в формате "x.x.x.x"
fn parse_ip(s: &str) -> Result<String, String> {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 4 {
        return Err(format!("Некорректный IP: '{}'. Формат: x.x.x.x", s));
    }
    for part in &parts {
        let num: u32 = part.parse().map_err(|_| format!("Часть '{}' не число", part))?;
        if num > 255 {
            return Err(format!("Часть '{}' > 255", part));
        }
    }
    Ok(s.to_string())
}

/// Кастомный парсер для email (простая проверка)
fn validate_email(s: &str) -> Result<String, String> {
    if s.contains('@') && s.contains('.') {
        Ok(s.to_string())
    } else {
        Err(format!("'{}' не похож на email (нет @ или .)", s))
    }
}

fn main() {
    let args = Args::parse();

    println!("✅ Возраст: {}", args.age);
    println!("✅ Вес: {} кг", args.weight);
    println!("✅ Режим: {:?}", args.mode);

    if let Some(ip) = args.ip {
        println!("✅ IP: {}", ip);
    }
    if let Some(email) = args.email {
        println!("✅ Email: {}", email);
    }
}

// Примеры запуска:
// cargo run --example 04_validation -- --age 25 --weight 80 --mode fast
// cargo run --example 04_validation -- --age 25 --ip 192.168.1.1 --email user@example.com
// cargo run --example 04_validation -- --age 300  ← ОШИБКА! (age может быть любой, но...)
// cargo run --example 04_validation -- --age 25 --weight 999  ← ОШИБКА! (weight > 300)
// cargo run --example 04_validation -- --age 25 --ip 256.1.1.1  ← ОШИБКА! (256 > 255)
