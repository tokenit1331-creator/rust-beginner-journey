// 🎯 ЗАДАНИЕ: Создай CLI для управления крипто-портфелем
//
// 💡 ADVICE: Это задание проверяет всё, что ты выучил:
// - Позиционные аргументы, опции, флаги 🏗️
// - ValueEnum для строгих списков 📋
// - Валидацию и кастомные парсеры ✅
// - Субкоманды 🎭
// - Продвинутые фичи (env, count, hide) ⚡
//
// Твоя задача — дописать структуру Cli так, чтобы код скомпилировался
// и работал как описано ниже.

use clap::{Parser, Subcommand, ValueEnum};

// 💡 ADVICE: Используй ValueEnum для типов операций.
// Это гарантирует, что пользователь не введёт "buyy" вместо "buy".
#[derive(Clone, ValueEnum)]
enum Operation {
    Buy,
    Sell,
    // 💡 HINT: Добавь сюда Staking с кастомным именем "stake"
}

// 💡 ADVICE: Токены — строгий список.
#[derive(Clone, ValueEnum)]
enum Token {
    Bitcoin,
    Ethereum,
    // 💡 HINT: Добавь Solana
}

// 💡 ADVICE: Структура портфеля — подсказка.
// Используй value_delimiter = ',' и num_args = 1.. для ввода
// нескольких токенов в формате: --portfolio "BTC 0.5"
struct PortfolioItem {
    token: Token,
    amount: f64,
}

// 💡 ADVICE: Вспомогательная функция для парсинга PortfolioItem
// Формат: "BTC 0.5" или "ETH 2.0"
fn parse_portfolio(s: &str) -> Result<PortfolioItem, String> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(format!("Неверный формат: {s}. Ожидается: TOKEN AMOUNT"));
    }

    let token = match parts[0].to_lowercase().as_str() {
        "btc" => Token::Bitcoin,
        "eth" => Token::Ethereum,
        _ => return Err(format!("Неизвестный токен: {}", parts[0])),
    };

    let amount: f64 = parts[1].parse()
        .map_err(|_| format!("Неверная сумма: {}", parts[1]))?;

    Ok(PortfolioItem { token, amount })
}

// ══════════════════════════════════════════
// 🎯 ТВОЯ ЗАДАЧА — допиши структуру Cli
// ══════════════════════════════════════════

/// Управление крипто-портфелем
///
/// Требования:
/// 1. Субкоманды: Trade, Portfolio, Help
/// 2. Trade принимает:
///    - --token / -t (ValueEnum: Bitcoin, Ethereum)
///    - --amount / -a (f64)
///    - --operation / -o (ValueEnum: Buy, Sell)
///    - --price / -p (f64, опционально)
///    - --verbose / -v (count)
/// 3. Portfolio принимает:
///    - список PortfolioItem через --items (num_args + value_parser)
///    - --api-key / -k (env = "PORTFOLIO_API_KEY")
///    - --format / -f (по умолчанию "table")
/// 4. Help — просто выводит справку
///
/// 💡 ADVICE: Начни с субкоманд.
/// Сделай enum Command с вариантами Trade, Portfolio, Help.
/// Потом для каждого варианта — отдельную структуру.
///
/// 💡 ADVICE: Не забудь subcommand_required = true.
/// Это защита от пустого запуска.
#[derive(Parser)]
#[command(name = "crypto-portfolio")]
struct Cli {
    // 💡 HINT: Добавь #[command(subcommand)]
    // и поле command: Command
}

// 💡 HINT: Определи enum Command с #[derive(Subcommand)]
// Варианты: Trade(TradeArgs), Portfolio(PortfolioArgs), Help

// 💡 HINT: Определи структуры TradeArgs, PortfolioArgs

// ══════════════════════════════════════════
// НЕ ТРОГАЙ main() — он уже готов
// ══════════════════════════════════════════

fn main() {
    let cli = Cli::parse();

    // match по командам
    // TODO: допиши match
    println!("🎯 Задание выполнено! (когда допишешь Cli)");
}

// 🧠 Подсказки:
// 1. cargo run --example exercise -- trade --token Bitcoin --amount 0.5 --operation Buy
// 2. cargo run --example exercise -- portfolio --items "BTC 0.5" "ETH 2.0" -k mykey
// 3. cargo run --example exercise -- help
//
// 💡 ADVICE: После завершения — проверь:
// - cargo build --example exercise (компилируется?)
// - cargo run --example exercise -- --help (красивый help?)
// - cargo run --example exercise -- trade --help (help по команде?)
// - Все ли комбинации аргументов работают?
//
// 🚀 Удачи!
