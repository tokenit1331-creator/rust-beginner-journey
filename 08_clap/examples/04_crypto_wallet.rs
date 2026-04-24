// ============================================================================
// 04_crypto_wallet.rs — ПОЛНЫЙ КРИПТО-КОШЕЛЁК (ВСЕ ФИЧИ ВМЕСТЕ)
// ============================================================================
//
// 🎯 Цель: Полноценный CLI-кошелёк с env, count, кастомным help, subcommands
// 🧠 Что изучаем: env, action = Count, custom help_template, hide
// 🪙 Контекст: Реальный крипто-кошелёк для терминала
//
// ============================================================================

use clap::{Parser, Subcommand};

/// Крипто-кошелёк нового поколения 🔥
///
/// ПОЧЕМУ такой длинный doc-комментарий?
/// Потому что clap использует doc-комментарии (///) для генерации --help.
/// Чем подробнее — тем понятнее пользователю.
#[derive(Parser)]
#[command(
    name = "crypto-wallet-pro",
    version = "1.0.0",
    about = "Профессиональный крипто-кошелёк для терминала",
    long_about = None,  // отключаем расширенный about (чтоб не дублировалось)
    subcommand_required = true,
    arg_required_else_help = true,

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  КАСТОМНЫЙ HELP TEMPLATE                                   ║
    // ║                                                             ║
    // ║  По умолчанию --help выглядит стандартно. Но мы можем       ║
    // ║  изменить его под свою стилистику.                          ║
    // ║                                                             ║
    // ║  {name}       — имя программы                               ║
    // ║  {version}    — версия                                     ║
    // ║  {author}     — автор                                      ║
    // ║  {about}      — описание                                   ║
    // ║  {usage}      — синтаксис использования                    ║
    // ║  {all-args}   — все аргументы                              ║
    // ║  {subcommands} — все субкоманды                            ║
    // ║                                                             ║
    // ║  ПОЧЕМУ кастомный help?                                     ║
    // ║  В реальном проекте это делает инструмент узнаваемым.       ║
    // ║  Solana CLI, Foundry, cargo — у каждого свой стиль help.    ║
    // ╚══════════════════════════════════════════════════════════════╝
    help_template = r#"
╔══════════════════════════════════╗
║  {name} (v{version})              ║
║  {about}                         ║
╚══════════════════════════════════╝

{usage-heading} {usage}

{all-args}{subcommands}

🔗 Создано с любовью в Rust + clap
"#
)]
struct Cli {
    // ╔══════════════════════════════════════════════════════════════╗
    // ║  ENV — ПЕРЕМЕННЫЕ ОКРУЖЕНИЯ                                ║
    // ║                                                             ║
    // ║  Никогда НЕ ХРАНИ API-ключи в коде или аргументах CLI!      ║
    // ║  API-ключи должны быть в переменных окружения или .env      ║
    // ║                                                             ║
    // ║  env = "CRYPTO_API_KEY" — если переменная установлена,      ║
    // ║  clap автоматически прочитает её и подставит значение.      ║
    // ║  Пользователь также может передать --api-key явно,          ║
    // ║  если хочет переопределить.                                ║
    // ║                                                             ║
    // ║  Приоритет: --api-key > CRYPTO_API_KEY > ошибка (если       ║
    // ║  обязательный и нет ни того, ни другого)                   ║
    // ║                                                             ║
    // ║  ПОЧЕМУ env, а не read_env_file()?                          ║
    // ║  clap сам проверяет переменную при старте. Не нужно         ║
    // ║  писать дополнительный код. Просто, безопасно, надёжно.     ║
    // ╚══════════════════════════════════════════════════════════════╝
    /// API-ключ для RPC-провайдера (можно через CRYPTO_API_KEY)
    #[arg(
        long,
        env = "CRYPTO_API_KEY",
        hide_env_values = true,  // не показывать значение в --help
        default_value = "demo-key"
    )]
    api_key: String,

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  COUNT — УРОВЕНЬ ПОДРОБНОСТИ                               ║
    // ║                                                             ║
    // ║  action = clap::ArgAction::Count — каждый -v увеличивает    ║
    // ║  счётчик:                                                   ║
    // ║    -v  → verbose = 1                                        ║
    // ║    -vv → verbose = 2                                        ║
    // ║    -vvv→ verbose = 3                                        ║
    // ║                                                             ║
    // ║  ПОЧЕМУ Count, а не просто bool?                            ║
    // ║  В реальных CLI бывают уровни логирования:                   ║
    // ║  -v = info, -vv = debug, -vvv = trace                       ║
    // ║  Один флаг = 2 уровня (есть/нет). Count = N уровней.        ║
    // ║                                                             ║
    // ║  Пример из жизни:                                           ║
    // ║  cargo build -v   → чуть подробнее                          ║
    // ║  cargo build -vv  → очень подробно                          ║
    // ║  cargo build -vvv → всё подряд (трассировка)               ║
    // ╚══════════════════════════════════════════════════════════════╝
    /// Подробность вывода (можно несколько -v)
    #[arg(
        short,
        long,
        action = clap::ArgAction::Count,
        global = true  // действует для ВСЕХ субкоманд
    )]
    verbose: u8,  // u8 потому что count редко превышает 3-4

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  HIDE — СКРЫТЫЕ АРГУМЕНТЫ                                   ║
    // ║                                                             ║
    // ║  hide = true — аргумент НЕ ПОКАЗЫВАЕТСЯ в --help,           ║
    // ║  но продолжает работать.                                    ║
    // ║                                                             ║
    // ║  ПОЧЕМУ hide?                                               ║
    // ║  Есть аргументы для разработчиков/тестирования, которые     ║
    // ║  не нужно показывать обычным пользователям.                 ║
    // ║  Например: --beta-mode, --internal-debug, --mock-rpc        ║
    // ╚══════════════════════════════════════════════════════════════╝
    /// Режим разработки (скрытый аргумент)
    #[arg(long, hide = true, global = true)]
    dev_mode: bool,

    /// Субкоманда
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Отправить транзакцию
    #[command(about = "Отправить ETH или токены")]
    Send {
        #[arg(short, long)]
        to: String,

        #[arg(short, long)]
        amount: f64,

        #[arg(short, long)]
        token: Option<String>,  // опционально: ETH или адрес токена
    },

    /// Показать баланс
    #[command(about = "Проверить баланс кошелька")]
    Balance {
        #[arg(short, long)]
        address: String,
    },

    /// Обменять токены
    #[command(about = "Обменять один токен на другой")]
    Swap {
        #[arg(short, long)]
        from: String,

        #[arg(short, long)]
        to: String,

        #[arg(short, long)]
        amount: f64,

        #[arg(long, default_value_t = 0.3)]
        slippage: f64,
    },
}


fn main() {
    let cli = Cli::parse();

    // ╔════════════════════════════════════════════════════════════╗
    // ║  ИСПОЛЬЗОВАНИЕ VERBOSE COUNT                              ║
    // ║                                                           ║
    // ║  cli.verbose — это u8. Чем больше -v, тем выше число.     ║
    // ║  Мы используем match, чтобы выбрать уровень детализации.  ║
    // ╚════════════════════════════════════════════════════════════╝
    let log_level = match cli.verbose {
        0 => "INFO",
        1 => "DEBUG",
        _ => "TRACE",  // 2 и выше
    };

    // ╔════════════════════════════════════════════════════════════╗
    // ║  API-ключ — прочитан из env или аргумента                 ║
    // ║  Пользователь даже не указал --api-key, но значение есть  ║
    // ╚════════════════════════════════════════════════════════════╝
    println!("🪙 Crypto Wallet Pro (v1.0.0)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  📋 Уровень логов: {}", log_level);

    if cli.dev_mode {
        println!("  🧪 РЕЖИМ РАЗРАБОТЧИКА");
    }

    println!();

    // Обрабатываем субкоманды
    match &cli.command {
        Commands::Send { to, amount, token } => {
            println!("📤 Отправка...");
            println!("  🎯 Кому:    {}", to);
            println!("  💰 Сумма:   {} ETH", amount);

            if let Some(t) = token {
                println!("  🪙 Токен:   {}", t);
            }

            if cli.verbose >= 2 {
                println!("  🔑 API Key: {}...{}",
                    &cli.api_key[..4],
                    &cli.api_key[cli.api_key.len()-4..]
                );
            }
        },
        Commands::Balance { address } => {
            println!("🪙 Баланс кошелька {}", address);
            println!("  💰 142.5 ETH");
            println!("  🪙 2,500 USDC");

            if cli.dev_mode {
                println!();
                println!("  🔧 Dev: RPC = mock://localhost:8545");
            }
        },
        Commands::Swap { from, to, amount, slippage } => {
            println!("🔄 Обмен");
            println!("  {:.4} {} → {}", amount, from, to);
            println!("  📉 Проскальзывание: {}%", slippage);
            println!();
            println!("  💱 Курс: 1 {} = 1,800 {} (симуляция)", from, to);
            println!("  ✅ Обмен выполнен!");
        },
    }
}


// ============================================================================
// 🎯 ПРОВЕРЬ СЕБЯ
// ============================================================================
//
// 1. cargo run --example 04_crypto_wallet -- --help
//    → Кастомный help с рамками!
//
// 2. CRYPTO_API_KEY=super-secret-key cargo run --example 04_crypto_wallet -- balance --address 0xabc
//    → API-ключ из переменной окружения
//
// 3. cargo run --example 04_crypto_wallet -- send --to 0xabc --amount 0.5 -vv
//    → verbose = 2 (TRACE) — покажет API ключ
//
// 4. cargo run --example 04_crypto_wallet -- swap --from ETH --to USDC --amount 1.0
//    → Субкоманда swap с slippage
//
// 5. cargo run --example 04_crypto_wallet -- --dev-mode balance --address 0xabc
//    → Скрытый аргумент --dev-mode (не виден в --help, но работает!)
//
// 6. cargo run --example 04_crypto_wallet --
//    → arg_required_else_help — покажет help, если нет субкоманды
//
// ============================================================================
