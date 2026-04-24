use clap::{Parser, ValueEnum};

// 💡 ADVICE: ValueEnum — это строгий список допустимых значений.
// Вместо того чтобы принимать String и потом проверять вручную,
// мы говорим clap: «Вот список сетей — всё остальное ошибка».
// Компилятор сам генерирует from_str() и список для --help.

/// Сети блокчейна, которые поддерживает наш кошелёк
#[derive(Clone, ValueEnum)]
enum Network {
    Ethereum,
    #[clap(name = "solana")]
    // 💡 ADVICE: name = "solana" — кастомное имя для CLI.
    // Без этого было бы "solana" (нижний регистр от названия варианта).
    // Полезно, если хочешь синонимы или другой регистр.
    Solana,
    Bitcoin,
}

/// Проверка баланса крипто-кошелька
#[derive(Parser)]
#[command(name = "check-balance")]
struct Cli {
    /// Адрес кошелька (0x + 40 hex-символов)
    #[arg(value_parser = parse_address)]
    // 💡 ADVICE: value_parser = parse_address — кастомный парсер.
    // Мы написали функцию, которая проверяет адрес.
    // Если она вернёт Err — clap сам покажет ошибку.
    // Не надо писать if-проверки в main()!
    address: String,

    /// Сеть блокчейна
    #[arg(short, long, value_enum, default_value = "ethereum")]
    // 💡 ADVICE: value_enum — указываем, что это ValueEnum.
    // default_value = "ethereum" — если сеть не указана, берём Ethereum.
    // clap сам проверит, что значение из списка.
    network: Network,

    /// Количество подтверждений для проверки (1-100)
    #[arg(short, long, default_value_t = 12, value_parser = clap::value_parser!(u32).range(1..=100))]
    // 💡 ADVICE: .range(1..=100) — валидация диапазона.
    // Если пользователь введёт 0 или 200 — clap скажет: "не в диапазоне".
    // Бесплатная проверка без единой строки кода.
    confirmations: u32,
}

// 💡 ADVICE: Функция-парсер — это обычная fn(&str) -> Result<T, String>.
// Она принимает строку из командной строки и возвращает
// либо готовое значение, либо сообщение об ошибке.
// clap сам вызовет её для каждого аргумента с value_parser.
//
// ПОЧЕМУ: Одна функция переиспользуется везде.
// Проверка адреса — в одном месте, а не размазана по коду.
fn parse_address(s: &str) -> Result<String, String> {
    // Проверяем длину адреса
    if s.len() != 42 {
        return Err(format!(
            "Неверная длина адреса: {} символов. Должно быть 42 (0x + 40 hex)",
            s.len()
        ));
    }

    // Проверяем префикс 0x
    if !s.starts_with("0x") {
        return Err(format!(
            "Адрес должен начинаться с 0x. Получено: {}...",
            &s[..4.min(s.len())]
        ));
    }

    // Проверяем, что остальные символы — hex
    let hex_part = &s[2..];
    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!(
            "Адрес содержит не-hex символы: {}",
            hex_part.chars().find(|c| !c.is_ascii_hexdigit()).unwrap()
        ));
    }

    // 💡 ADVICE: Возвращаем адрес в нижнем регистре.
    // Это нормализация — чтобы 0xABC и 0xabc были одинаковыми.
    Ok(s.to_lowercase())
}

fn main() {
    let args = Cli::parse();

    println!("🔍 Проверка баланса...");
    println!("  Адрес:     {}", args.address);
    println!("  Сеть:      {:?}", args.network);
    println!("  Подтверждений: {}", args.confirmations);

    // Имитация запроса к блокчейну
    let balance = match args.network {
        Network::Ethereum => 42.5,
        Network::Solana => 128.0,
        Network::Bitcoin => 0.5,
    };

    println!("💰 Баланс: {} ETH (в сети {:?})", balance, args.network);
    println!("✅ Проверка завершена");

    // 💡 ADVICE: Если бы адрес был неверный — мы бы сюда не попали.
    // clap вызвал бы exit(2) с сообщением об ошибке.
    // Пользователь увидел бы:
    //   error: Invalid value for 'address': Неверная длина адреса: 5 символов.
}

// 🧠 ИТОГ ШАГА 2:
// - ValueEnum — строгий список значений (Ethereum, Solana, Bitcoin)
// - Кастомный парсер parse_address — проверка и нормализация
// - .range() — диапазон для чисел
// - Пользователь не может передать неверные данные — clap проверяет всё до main()
