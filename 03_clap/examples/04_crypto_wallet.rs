use clap::Parser;

// 💡 ADVICE: Этот пример собирает ВСЕ продвинутые фичи вместе.
// env, Count, hide, кастомный help_template — то, что отличает
// любительский CLI от профессионального.

/// Полноценный крипто-кошелёк
///
/// ПОЧЕМУ кастомный help_template:
/// - Брендирование: пользователь видит название и контакты
/// - Секция after_help: ссылки на сайт и поддержку
/// - Профессиональный вид как у aws, kubectl, gh
#[derive(Parser)]
#[command(
    name = "crypto-wallet",
    version = "1.0.0",
    about = "Полноценное управление крипто-кошельком",
    long_about = "Crypto Wallet CLI — профессиональный инструмент для работы
с крипто-кошельками. Поддерживает Ethereum, Solana и Bitcoin.
Используется в production в компании CryptoCorp.",
    help_template = r"""
╔══════════════════════════════════════════════════╗
║  {name} v{version}                              ║
║  {about}                                        ║
╚══════════════════════════════════════════════════╝

{usage-heading} {usage}

{all-args}{after-help}

🔗 Документация: https://docs.cryptocorp.com
📧 Поддержка: support@cryptocorp.com
🌐 Сайт: https://cryptocorp.com

"""
)]
struct Cli {
    /// API-ключ для доступа к блокчейн-ноде
    ///
    /// 💡 ADVICE: env — ключ берётся из переменной окружения.
    /// Если не указать в командной строке — clap прочитает CRYPTO_API_KEY.
    /// Если нет ни там, ни там — ошибка.
    ///
    /// ПОЧЕМУ env, а не аргумент?
    /// 1. Ключ не светится в ps aux и истории bash
    /// 2. Можно хранить в .env (добавить в .gitignore)
    /// 3. CI/CD: передаётся через secrets, а не в команде
    #[arg(short, long, env = "CRYPTO_API_KEY")]
    api_key: String,

    /// Адрес кошелька
    #[arg(short, long, value_parser = parse_address)]
    address: String,

    /// Уровень детализации (-v, -vv, -vvv)
    ///
    /// 💡 ADVICE: action = clap::ArgAction::Count — гениальная штука.
    /// Вместо трёх флагов -v, -vv, -vvv — один, который считает повторы.
    /// Уровни: 0 (тихо), 1 (-v), 2 (-vv), 3+ (-vvv и выше).
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Секретный режим для дампа состояния (скрыт из help)
    ///
    /// 💡 ADVICE: hide = true — опция существует, но не показывается в --help.
    /// Нужно для:
    /// - отладочных режимов (только для разработчиков)
    /// - A/B тестирования фич
    /// - обратной совместимости (старый флаг, который не хотим рекламировать)
    #[arg(long, hide = true)]
    dump_state: bool,
}

fn parse_address(s: &str) -> Result<String, String> {
    if s.len() != 42 || !s.starts_with("0x") {
        return Err(format!("Неверный адрес: {s}"));
    }
    Ok(s.to_lowercase())
}

fn main() {
    let args = Cli::parse();

    // 💡 ADVICE: match по verbose — каноничный способ обработки уровней.
    // 0 — тихо, 1 — инфо, 2 — дебаг, 3+ — трейс.
    match args.verbose {
        0 => println!("🔐 Кошелёк: {}", &args.address[..10]),
        1 => {
            println!("🔐 Кошелёк: {}", args.address);
            println!("  API-ключ: {}...", &args.api_key[..4]);
            println!("  Режим: продакшн");
        }
        2 => {
            println!("🔐 Кошелёк: {}", args.address);
            println!("  API-ключ: {}...", &args.api_key[..4]);
            println!("  Режим: продакшн");
            println!("  Версия CLI: 1.0.0");
            println!("  Rust version: {}", rustc_version());
        }
        _ => {
            // -vvv и выше — полный дамп
            println!("🔐 Кошелёк: {}", args.address);
            println!("  API-ключ: {}...", &args.api_key[..4]);
            println!("  Режим: продакшн");
            println!("  Версия CLI: 1.0.0");
            println!("  Rust version: {}", rustc_version());
            println!("  Debug: {:?}", args);
        }
    }

    // 💡 ADVICE: dump_state — скрытая опция для дебага.
    // Пользователь её не видит в help, но разработчик может использовать.
    if args.dump_state {
        println!("\n📋 Дамп состояния:");
        println!("  API ключ загружен: {}", !args.api_key.is_empty());
        println!("  Адрес: {}", args.address);
        println!("  Адрес валидный: {}", args.address.len() == 42 && args.address.starts_with("0x"));
        println!("  Уровень логов: {}", args.verbose);
    }

    println!("✅ Кошелёк инициализирован");
}

fn rustc_version() -> String {
    format!("1.75.0 (стабильная)")
}

// 🧠 ИТОГ ШАГА 4:
// - env — безопасное получение API-ключей
// - Count — элегантные уровни логирования
// - hide — скрытые опции для разработчиков
// - help_template — профессиональный брендированный help
// - Всё вместе — production-ready CLI
