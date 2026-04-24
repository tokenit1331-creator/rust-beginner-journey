use clap::{Parser, Subcommand};

// 💡 ADVICE: #[derive(Subcommand)] — это enum, где каждый вариант — отдельная команда.
// Вместо того чтобы делать --send, --balance, --history как флаги,
// мы делаем их субкомандами: crypto-tx send ... / crypto-tx balance ...
//
// В реальном мире: cargo, git, npm, docker — все используют субкоманды.
// Это стандарт для CLI с несколькими действиями.

/// Отправка транзакций в блокчейне
#[derive(Parser)]
#[command(
    name = "crypto-tx",
    version = "1.0",
    about = "Отправка и управление транзакциями",
    subcommand_required = true
    // 💡 ADVICE: subcommand_required = true — защита от пустого запуска.
    // Если пользователь запустит без команды — clap покажет help.
    // Без этого флага программа бы просто завершилась молча.
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Отправить токены на адрес
    ///
    /// ПОЧЕМУ: Каждая команда — отдельная структура.
    /// Send(SendArgs) — агрегация (has-a), а не наследование (is-a).
    /// SendArgs — это отдельный тип, который можно тестировать и переиспользовать.
    Send(SendArgs),

    /// Проверить баланс кошелька
    Balance(BalanceArgs),

    /// Показать историю транзакций
    History(HistoryArgs),
}

#[derive(Parser)]
struct SendArgs {
    /// Адрес получателя (0x + 40 hex)
    #[arg(short, long, value_parser = parse_address)]
    to: String,

    /// Сумма для отправки
    #[arg(short, long)]
    amount: f64,

    /// Отправить ВСЁ (конфликтует с --amount)
    ///
    /// 💡 ADVICE: conflicts_with = "amount" — защита от глупых ошибок.
    // Если пользователь указал --all, то --amount нельзя.
    // clap сам скажет: "error: the argument '--all' cannot be used with '--amount'"
    // Без этого пришлось бы писать if-проверку вручную.
    #[arg(long, conflicts_with = "amount")]
    all: bool,

    /// Комментарий к транзакции (теги)
    #[arg(short, long, num_args = 0..=5)]
    // 💡 ADVICE: num_args = 0..=5 — от 0 до 5 значений.
    // Без num_args Vec<String> ждёт ровно одно значение.
    // С num_args можно: cargo run -- send --to 0x... --tags urgent high gas
    tags: Vec<String>,
}

#[derive(Parser)]
struct BalanceArgs {
    /// Адрес кошелька
    #[arg(short, long, value_parser = parse_address)]
    address: String,
}

#[derive(Parser)]
struct HistoryArgs {
    /// Адрес кошелька
    #[arg(short, long, value_parser = parse_address)]
    address: String,

    /// Количество последних транзакций
    #[arg(short, long, default_value_t = 10)]
    limit: u32,
}

fn parse_address(s: &str) -> Result<String, String> {
    if s.len() != 42 || !s.starts_with("0x") {
        return Err(format!("Неверный адрес: {s}"));
    }
    Ok(s.to_lowercase())
}

fn main() {
    let cli = Cli::parse();

    // 💡 ADVICE: match по командам — короткий и читаемый.
    // Каждая команда — отдельный match arm.
    // Если добавить новую команду — компилятор скажет: "match не exhaustive".
    // Безопасно и удобно.
    match &cli.command {
        Command::Send(args) => {
            println!("📤 Отправка транзакции...");
            println!("  Кому:      {}", args.to);
            if args.all {
                println!("  Сумма:     ВСЁ");
            } else {
                println!("  Сумма:     {} токенов", args.amount);
            }
            if !args.tags.is_empty() {
                println!("  Теги:      {}", args.tags.join(", "));
            }

            // Симуляция отправки
            println!("✅ Транзакция отправлена! Hash: 0x{}", "a1b2c3d4");
        }
        Command::Balance(args) => {
            println!("💰 Баланс: {} → 42.5 ETH", args.address);
        }
        Command::History(args) => {
            println!("📜 История для {}", args.address);
            println!("  Показано последних {} транзакций", args.limit);
            for i in 1..=args.limit.min(5) {
                println!("  {}. Отправка 0.5 ETH → 0x... (вчера)", i);
            }
        }
    }
}

// 🧠 ИТОГ ШАГА 3:
// - Субкоманды (Send, Balance, History) — стандарт для CLI с несколькими действиями
// - Каждая команда — отдельная структура (модульность)
// - conflicts_with — защита от несовместимых флагов
// - subcommand_required — защита от пустого запуска
// - num_args — гибкое количество значений
