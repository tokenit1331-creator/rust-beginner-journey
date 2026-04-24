// ============================================================================
// exercise.rs — ТВОЁ ЗАДАНИЕ: СОЗДАЙ СВОЙ КРИПТО-CLI 🚀
// ============================================================================
//
// 🎯 Цель: Применить всё, что ты изучил
// ⏱  Время: 30-60 минут
// 📚 Что нужно: уроки 01-04
//
// ============================================================================

// ╔══════════════════════════════════════════════════════════════════╗
// ║  ЗАДАНИЕ: Крипто-кошелёк с портфелем                            ║
// ║                                                                 ║
// ║  Создай CLI-инструмент crypto-portfolio, который умеет:         ║
// ║                                                                 ║
// ║  1. Показывать портфель (balance)                               ║
// ║     crypto-portfolio balance --address 0xabc --network ethereum ║
// ║                                                                 ║
// ║  2. Добавлять актив (add-token)                                 ║
// ║     crypto-portfolio add-token --symbol ETH --amount 1.5        ║
// ║                                                                 ║
// ║  3. Удалять актив (remove-token)                                ║
// ║     crypto-portfolio remove-token --symbol ETH --all            ║
// ║     или                                                         ║
// ║     crypto-portfolio remove-token --symbol ETH --amount 0.5     ║
// ║     (--all и --amount конфликтуют!)                             ║
// ║                                                                 ║
// ║  4. Показывать портфель в USD (portfolio --usd)                 ║
// ║                                                                 ║
// ║  5. ★ Субкоманда history с флагом --verbose (Count)             ║
// ║     crypto-portfolio history --days 30 -vv                     ║
// ║                                                                 ║
// ║  6. ★★ API-ключ из переменной окружения PORTFOLIO_API_KEY      ║
// ║                                                                 ║
// ║  7. ★★★ Кастомный парсер адреса (0x + 40 hex символов)        ║
// ║                                                                 ║
// ║  8. ★★★★ Кастомный help_template с именем и версией           ║
// ║                                                                 ║
// ╚══════════════════════════════════════════════════════════════════╝

// ╔══════════════════════════════════════════════════════════════════╗
// ║  ПОРТФОЛИО (данные для имитации)                                ║
// ║                                                                 ║
// ║  Это структура данных, которая будет хранить твой портфель.     ║
// ║  Не трогай её — используй в своём коде.                        ║
// ╚══════════════════════════════════════════════════════════════════╝
struct Portfolio {
    tokens: Vec<TokenHolding>,
}

struct TokenHolding {
    symbol: String,
    amount: f64,
    price_usd: f64,
}

impl Portfolio {
    fn new() -> Self {
        Portfolio {
            tokens: vec![
                TokenHolding { symbol: "ETH".into(), amount: 1.5, price_usd: 3500.0 },
                TokenHolding { symbol: "BTC".into(), amount: 0.1, price_usd: 65000.0 },
                TokenHolding { symbol: "SOL".into(), amount: 50.0, price_usd: 150.0 },
                TokenHolding { symbol: "USDC".into(), amount: 1000.0, price_usd: 1.0 },
            ],
        }
    }

    fn total_usd(&self) -> f64 {
        self.tokens.iter().map(|t| t.amount * t.price_usd).sum()
    }

    fn find_token(&self, symbol: &str) -> Option<&TokenHolding> {
        self.tokens.iter().find(|t| t.symbol.eq_ignore_ascii_case(symbol))
    }

    fn find_token_mut(&mut self, symbol: &str) -> Option<&mut TokenHolding> {
        self.tokens.iter_mut().find(|t| t.symbol.eq_ignore_ascii_case(symbol))
    }
}


// ╔══════════════════════════════════════════════════════════════════╗
// ║  ТВОЯ ЗАДАЧА:                                                  ║
// ║                                                                 ║
// ║  Создай структуру Cli с #[derive(Parser)] и enum Commands       ║
// ║  с #[derive(Subcommand)]. Используй всё, что изучил:            ║
// ║                                                                 ║
// ║  ✅ #[derive(Parser)]                                          ║
// ║  ✅ #[derive(Subcommand)]                                       ║
// ║  ✅ #[arg(short, long)]                                         ║
// ║  ✅ #[arg(default_value_t)]                                     ║
// ║  ✅ #[arg(conflicts_with)]                                      ║
// ║  ✅ #[arg(action = clap::ArgAction::Count)]                     ║
// ║  ✅ #[arg(env = "...")]                                        ║
// ║  ✅ #[arg(value_parser = ...)]                                  ║
// ║  ✅ #[command(help_template = ...)]                             ║
// ║  ✅ #[arg(hide = true)]                                         ║
// ╚══════════════════════════════════════════════════════════════════╝

// ╔══════════════════════════════════════════════════════════════════╗
// ║  ПОДСКАЗКА (разверни, если нужна помощь):                       ║
// ║                                                                 ║
// ║  Структура должна выглядеть примерно так:                       ║
// ║                                                                 ║
// ║  #[derive(Parser)]                                              ║
// ║  #[command(name = "crypto-portfolio", ...)]                     ║
// ║  struct Cli {                                                    ║
// ║      #[arg(long, env = "PORTFOLIO_API_KEY")]                    ║
// ║      api_key: String,                                            ║
// ║                                                                  ║
// ║      #[arg(short, long, action = clap::ArgAction::Count)]        ║
// ║      verbose: u8,                                                ║
// ║                                                                  ║
// ║      #[command(subcommand)]                                      ║
// ║      command: Commands,                                          ║
// ║  }                                                               ║
// ║                                                                  ║
// ║  enum Commands {                                                 ║
// ║      Balance { ... },                                            ║
// ║      AddToken { ... },                                           ║
// ║      RemoveToken { ... },                                        ║
// ║      History { ... },                                            ║
// ║  }                                                               ║
// ╚══════════════════════════════════════════════════════════════════╝


#[allow(unused_imports)]
use clap::{Parser, Subcommand};

// ╔══════════════════════════════════════════════════════════════════╗
// ║  НАПИШИ СВОЙ КОД ЗДЕСЬ 👇                                       ║
// ╚══════════════════════════════════════════════════════════════════╝

// ТВОЯ СТРУКТУРА Cli
// ...

// ТВОИ СУБКОМАНДЫ
// ...

// ФУНКЦИЯ ПАРСЕРА АДРЕСА (если нужна)
// ...


fn main() {
    // ╔══════════════════════════════════════════════════════════════╗
    // ║  РАСКОММЕНТИРУЙ ЭТУ СТРОКУ, КОГДА БУДЕШЬ ГОТОВ:              ║
    // ║  let cli = Cli::parse();                                     ║
    // ╚══════════════════════════════════════════════════════════════╝

    println!("📂 Крипто-портфель");
    println!("━━━━━━━━━━━━━━━━━");
    println!("💡 Открой файл examples/exercise.rs и выполни задание!");

    // ╔══════════════════════════════════════════════════════════════╗
    // ║  ДЛЯ ПРОВЕРКИ СВОЕГО РЕШЕНИЯ:                               ║
    // ║                                                             ║
    // ║  cargo run --example exercise -- --help                      ║
    // ║  cargo run --example exercise -- balance --address 0xabc     ║
    // ║  cargo run --example exercise -- add-token --symbol ADA ...  ║
    // ║                                                             ║
    // ║  Запускай cargo check после каждого изменения:              ║
    // ║  cargo check --example exercise                              ║
    // ╚══════════════════════════════════════════════════════════════╝
}


// ============================================================================
// ⚠️  КРИТЕРИИ ПРОВЕРКИ (чекни себя)
// ============================================================================
//
//  1. balance — принимает address (с парсером) и network (с default)
//  2. add-token — принимает symbol, amount, price_usd
//  3. remove-token — принимает symbol, ИЛИ --all ИЛИ --amount (конфликт!)
//  4. history — принимает --days (default 30), --verbose (Count)
//  5. portfolio --usd — показывает общую сумму в USD
//  6. API_KEY из env
//  7. Кастомный help с именем и версией
//  8. --dev-mode скрыт из help
//
//  Справился? Ты ОФИЦИАЛЬНО прошёл урок clap! 🎉
//
// ============================================================================
