use clap::Parser;

// 💡 ADVICE: #[derive(Parser)] — это магия, которую можно использовать.
// Она генерирует ВСЁ: парсинг, --help, --version, проверку типов.
// Не надо писать match по argv вручную — это делает clap за тебя.

/// Простой крипто-терминал
///
/// ПОЧЕМУ: Документация в /// попадает в --help автоматически.
/// Это называется doc-comments. clap их подхватывает.
#[derive(Parser)]
#[command(
    name = "crypto-terminal",
    version = "1.0",
    about = "Терминал для работы с крипто-кошельком"
    // 💡 ADVICE: name, version, about — это метаданные.
    // Они попадают в --help и --version.
    // Не пиши их в println! — clap сделает это сам.
)]
struct Cli {
    /// Адрес кошелька (0x + 40 hex-символов)
    ///
    /// ПОЧЕМУ: Это позиционный аргумент — без флага.
    /// Пользователь пишет: cargo run -- 0x123... вместо --address 0x123...
    /// Удобно для обязательных параметров, которые нужны всегда.
    address: String,

    /// Сеть блокчейна
    ///
    /// ПОЧЕМУ: short = 'n' — одна буква, long = "network" — полное имя.
    /// В --help будет: -n <NETWORK>, --network <NETWORK>
    /// short определяется вручную, чтобы не было конфликтов с другими флагами.
    #[arg(short = 'n', long = "network", default_value = "ethereum")]
    // 💡 ADVICE: default_value — покажи пользователю значение по умолчанию в --help.
    // Если сеть не указана — берётся ethereum. Удобно для частого сценария.
    network: String,

    /// Подробный вывод
    ///
    /// ПОЧЕМУ: short и long без букв — clap сам подставит (-v и --verbose).
    /// Для bool-флагов это идеально: есть/нет.
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    // 💡 ADVICE: .parse() сам вызывает exit(2) при ошибке + печатает help.
    // НЕ надо оборачивать в match — это лишний код.
    // Если пользователь ошибся — clap сам скажет как правильно.
    let args = Cli::parse();

    // Выводим информацию
    println!("🔐 Адрес кошелька: {}", args.address);
    println!("⛓️  Сеть: {}", args.network);

    // 💡 ADVICE: Флаги bool — это true/false.
    // Не надо писать match или if args.verbose == "true".
    // clap сам превращает --verbose в true.
    if args.verbose {
        println!("🔍 Подробный режим включён");
        println!("  - Полный адрес: {}", args.address);
        println!("  - Длина адреса: {} символов", args.address.len());
        println!("  - Начинается с 0x: {}", args.address.starts_with("0x"));
    }

    println!("✅ Готово!");
}

// 🧠 ИТОГ ШАГА 1:
// - Позиционные аргументы (address) — без флага, обязательные
// - Опции (--network) — с именем, можно пропустить (есть default)
// - Флаги (--verbose) — bool, есть/нет
// - --help и --version — сгенерированы автоматически
// - doc-comments (///) — попали в help
