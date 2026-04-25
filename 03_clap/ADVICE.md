# 💡 Советы от опытного Rust-разработчика (по clap)

> *Ошибки, которые делают все новички, и как их избежать*

---

## 📋 Содержание

| № | Совет | О чём |
|---|-------|-------|
| 1 | **Derive — всегда** | Почему Derive побеждает Builder в 95% случаев |
| 2 | **unwrap() — это нормально** | Когда можно, а когда нельзя |
| 3 | **ValueEnum вместо магии строк** | Компилятор — твой друг |
| 4 | **default_value_t спасает** | Меньше кода = меньше багов |
| 5 | **Субкоманды — modular design** | Не пиши монстров |
| 6 | **env для ключей** | Безопасность с рождения |
| 7 | **Count для verbose** | Элегантнее, чем флаги |
| 8 | **conflicts_with — подстраховка** | Там, где компилятор бессилен |
| 9 | **Кастомный парсер — DRY** | Не повторяйся |
| 10 | **Тестируй CLI** | Как тестировать argv |
| 11 | **help_template — твой бренд** | Впечатление пользователя |
| 12 | **num_args — для списков** | Правильный способ |
| 13 | **hide — для дебага** | Скрывай, а не удаляй |
| 14 | **trailing_var_arg для файлов** | Работа с `--` |
| 15 | **ArgGroup — сложная логика** | Когда conflicts_with не хватает |
| 16 | **flatten — переиспользование** | DRY для структур |
| 17 | **value_delimiter** | CSV-стиль |
| 18 | **subcommand_required** | Защита от пустого запуска |

---

## 🔥 Совет №1: Derive — всегда, Builder — только когда ОЧЕНЬ нужно

### Что делают новички
```rust
// ❌ Пишут Builder, потому что «это настоящий код без магии»
let matches = Command::new("myapp")
    .arg(Arg::new("name").required(true))
    .get_matches();
```

### Что делают опытные
```rust
// ✅ Derive — меньше кода, ошибки на этапе компиляции
#[derive(Parser)]
struct Args {
    /// Имя пользователя
    name: String,
}
```

### 🤔 Почему?
| Критерий | Derive | Builder |
|---|---|---|
| Ошибки | **На этапе компиляции** ✅ | В рантайме ❌ |
| Кода писать | **В 3 раза меньше** | Много |
| Читаемость | **Структура видна сразу** | Размазано |
| Гибкость | 95% случаев | 5% экзотики |

**Вывод:** Используй Derive. Builder — только если нужно динамически строить аргументы (например, читать конфиг и добавлять опции из файла).

---

## 🔥 Совет №2: unwrap() в main() — это НОРМАЛЬНО

### Что делают новички
```rust
// ❌ Боятся unwrap и пишут монстров
fn main() {
    let args = match Cli::parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Ошибка: {e}");
            std::process::exit(1);
        }
    };
}
```

### Что делают опытные
```rust
// ✅ clap сам вызывает exit(2) при ошибке + выводит help
fn main() {
    let args = Cli::parse();  // Если ошибка — программа сама завершится
}
```

### 🤔 Почему?
Метод `.parse()` сам вызывает `process::exit(2)` и печатает ошибку с `--help`. Оборачивать в `match` — лишняя работа. **Исключение:** если ты пишешь библиотеку, а не бинарник — там используй `TryFrom`.

---

## 🔥 Совет №3: ValueEnum вместо raw String

### Что делают новички
```rust
// ❌ Потом придётся писать match с обработкой ошибок
#[arg(short, long)]
network: String,  // пользователь может ввести "solana", "SolanA", "SOL"
```

### Что делают опытные
```rust
// ✅ Компилятор сам проверит + автодополнение в help
#[derive(ValueEnum, Clone)]
enum Network {
    Ethereum,
    Solana,
    Bitcoin,
}

#[arg(short, long, value_enum)]
network: Network,
```

### 🤔 Почему?
1. Компилятор отлавливает опечатки — никаких `"Sollana"` в рантайме
2. `--help` показывает список: `[possible values: ethereum, solana, bitcoin]`
3. clap сам делает `.parse()` — не пишешь `from_str()`
4. Shell completion работает из коробки

---

## 🔥 Совет №4: default_value_t — пиши меньше кода

### Что делают новички
```rust
// ❌ Option + match + unwrap_or
#[arg(short, long)]
count: Option<u32>,

fn main() {
    let count = args.count.unwrap_or(1);
}
```

### Что делают опытные
```rust
// ✅ Всё в одном атрибуте
#[arg(short, long, default_value_t = 1)]
count: u32,

fn main() {
    let count = args.count;  // уже u32, не Option
}
```

### 🤔 Почему?
- Меньше кода → меньше мест для багов
- `--help` сразу показывает `[default: 1]` — пользователь видит дефолт
- Не нужно писать `.unwrap_or()` в каждом месте использования

---

## 🔥 Совет №5: Субкоманды — разбивай логику

### Что делают новички
```rust
// ❌ Огромный enum, match на 100 строк
enum Command {
    Send { to: String, amount: f64 },
    Balance { address: String, network: String },
}
```

### Что делают опытные
```rust
// ✅ Каждая команда — отдельная структура
#[derive(Subcommand)]
enum Command {
    /// Отправить токены
    Send(SendArgs),
    /// Проверить баланс
    Balance(BalanceArgs),
}

#[derive(Args)]
struct SendArgs {
    /// Адрес получателя
    #[arg(short, long)]
    to: String,
    /// Сумма
    #[arg(short, long)]
    amount: f64,
}

#[derive(Args)]
struct BalanceArgs {
    /// Адрес кошелька
    address: String,
}
```

### 🤔 Почему?
- Каждая команда — отдельная структура → легко тестировать
- Можно переиспользовать `SendArgs` в других частях программы
- `match command` остаётся коротким и читаемым
- Добавление новой команды = новая структура + один вариант в enum

---

## 🔥 Совет №6: env для API-ключей — безопасность

### Что делают новички
```rust
// ❌ Ключ в аргументах — светится в ps aux и истории bash
#[arg(short, long)]
api_key: String,
```

### Что делают опытные
```rust
// ✅ Ключ из переменной окружения — не светится
#[arg(short, long, env = "CRYPTO_API_KEY")]
api_key: String,
```

### 🤔 Почему?
- Аргументы командной строки видны в `ps aux` всем процессам
- История bash хранит все команды — ключ останется в файле
- `.env` файл можно добавить в `.gitignore`
- CI/CD: ключи передаются через secrets, а не через args

---

## 🔥 Совет №7: Count для verbose — -v, -vv, -vvv

### Что делают новички
```rust
// ❌ Три отдельных флага
#[arg(short = 'v', long = "verbose")]
verbose: bool,
```

### Что делают опытные
```rust
// ✅ Count — одно поле, несколько уровней
#[arg(short, long, action = clap::ArgAction::Count)]
verbose: u8,

fn main() {
    match args.verbose {
        0 => println!("Обычный режим"),
        1 => println!("Подробный режим (-v)"),
        2 => println!("Очень подробный (-vv)"),
        _ => println!("Отладка (-vvv и выше)"),
    }
}
```

### 🤔 Почему?
- Одно поле вместо трёх
- `-v -v -v` → `verbose = 3`
- Красиво и лаконично
- В коммерческих CLI (aws, kubectl, gh) везде так

---

## 🔥 Совет №8: conflicts_with и requires — подстраховка

### Что делают новички
```rust
// ❌ Проверка вручную
fn main() {
    if args.all && args.amount.is_some() {
        eprintln!("--all и --amount нельзя вместе");
        std::process::exit(1);
    }
}
```

### Что делают опытные
```rust
// ✅ clap сам проверяет — код чище
#[arg(long, conflicts_with = "amount")]
all: bool,

#[arg(long)]
amount: Option<f64>,
```

### 🤔 Почему?
- clap проверяет до выполнения твоего кода — ошибка видна сразу
- Не нужно писать if-проверки в main()
- Сообщение об ошибке от clap понятнее: `error: the argument '--all' cannot be used with '--amount'`

---

## 🔥 Совет №9: Кастомный парсер — DRY (Don't Repeat Yourself)

### Что делают новички
```rust
// ❌ Проверка адреса в каждом месте
fn main() {
    if args.to.len() != 42 || !args.to.starts_with("0x") {
        eprintln!("Неверный адрес");
    }
}
```

### Что делают опытные
```rust
// ✅ Одна функция — переиспользуется везде
fn parse_address(s: &str) -> Result<String, String> {
    if s.len() != 42 || !s.starts_with("0x") {
        return Err(format!("Неверный адрес: {s}. Должен быть 0x + 40 hex-символов"));
    }
    Ok(s.to_lowercase())
}

#[arg(value_parser = parse_address)]
to: String,

#[arg(value_parser = parse_address)]
from: String,
```

### 🤔 Почему?
- Одна точка правды для валидации
- clap сам показывает ошибку в формате: `error: Invalid value '0xabc' for '--to': Неверный адрес`
- Парсер можно переиспользовать в тестах
- Форматирование (`to_lowercase()`) применяется сразу — не надо помнить

---

## 🔥 Совет №10: Тестируй CLI (но не руками)

### Что делают новички
```bash
# ❌ Тест руками
cargo run -- --invalid-flag
# смотрят, что упало
```

### Что делают опытные
```rust
// ✅ assert_cmd — реальные тесты в cargo test
use assert_cmd::Command;

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("mycli").unwrap();
    cmd.arg("--help")
       .assert()
       .success()
       .stdout(predicates::str::contains("Usage"));
}

#[test]
fn test_invalid_address() {
    let mut cmd = Command::cargo_bin("mycli").unwrap();
    cmd.args(["balance", "--address", "invalid"])
       .assert()
       .failure()
       .stderr(predicates::str::contains("Неверный адрес"));
}
```

### 🤔 Почему?
- Тесты запускаются одной командой: `cargo test`
- Не нужно руками вводить команды
- CI/CD ловит регрессии
- Документация: тесты показывают, как должно работать

---

## 🔥 Совет №11: help_template — твой бренд

### Что делают новички
```rust
// ❌ Стандартный help — скучно
#[command(author, version, about, long_about = None)]
```

### Что делают опытные
```rust
// ✅ Кастомный шаблон — профессионально
#[command(
    name = "crypto-wallet",
    version,
    about = "Управление крипто-кошельком",
    help_template = "{name} v{version}
{author-with-newline}{about-with-newline}

{usage-heading} {usage}

{all-args}{after-help}

Сайт: https://mycryptoproject.com
Вопросы: support@mycryptoproject.com
"
)]
```

### 🤔 Почему?
- Пользователь видит профессиональный инструмент
- Можно добавить ссылки, контакты, примеры использования
- Отличает твой проект от поделок новичков

---

## 🔥 Совет №12: num_args для списков, а не нескольких флагов

### Что делают новички
```rust
// ❌ Три флага для списка
#[arg(short = 't')]
tag1: Option<String>,
#[arg(short = 'T')]
tag2: Option<String>,
```

### Что делают опытные
```rust
// ✅ Один флаг — много значений
#[arg(short, long, num_args = 0..=5)]
tags: Vec<String>,
// Использование: --tags urgent high gas
```

### 🤔 Почему?
- Любое количество значений (0, 1, 5)
- Порядок сохраняется
- Не нужно знать, сколько будет тегов заранее

---

## 🔥 Совет №13: hide — для скрытых опций (дебаг/тест)

```rust
// Скрытая опция для тестирования — не светится в --help
#[arg(long, hide = true)]
debug_mode: bool,
```

### 🤔 Почему?
- Нужно для интеграционных тестов
- Не путает пользователя лишними флагами
- Можно включить в документации, но скрыть из --help

---

## 🔥 Совет №14: trailing_var_arg для --

```rust
// После -- всё, что угодно (файлы)
#[arg(last = true)]
files: Vec<String>,
// Использование: crypto-wallet -- file1.txt file2.txt
```

### 🤔 Почему?
- Стандарт Unix: `--` означает «дальше не флаги»
- Пользователи ожидают такого поведения
- Безопасно: файлы не будут интерпретированы как флаги

---

## 🔥 Совет №15: ArgGroup — сложная логика

```rust
use clap::ArgGroup;

// Группа: нужен хотя бы один из {csv, json, table}
#[command(group = ArgGroup::new("format")
    .required(true)
    .args(["csv", "json", "table"]))]
struct Args {
    #[arg(long)]
    csv: bool,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    table: bool,
}
```

### 🤔 Почему?
- `conflicts_with` — только бинарные конфликты
- `ArgGroup` — любые комбинации: «хотя бы один», «ровно один», «все»
- Сообщение об ошибке: `error: one of the arguments '--csv' or '--json' or '--table' is required`

---

## 🔥 Совет №16: flatten — переиспользуй структуры

```rust
// Общие опции для всех команд
#[derive(Args)]
struct CommonArgs {
    #[arg(short, long, env = "CRYPTO_API_KEY")]
    api_key: String,
    #[arg(short, long, default_value_t = String::from("mainnet"))]
    network: String,
}

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    common: CommonArgs,
    #[command(subcommand)]
    command: Command,
}
```

### 🤔 Почему?
- DRY: общие опции в одном месте
- Легко добавить общий флаг для всех команд
- Тестируешь `CommonArgs` один раз

---

## 🔥 Совет №17: value_delimiter для CSV-стиля

```rust
#[arg(short, long, value_delimiter = ',')]
tokens: Vec<String>,
// Использование: --tokens BTC,ETH,SOL
// Результат: ["BTC", "ETH", "SOL"]
```

### 🤔 Почему?
- Короче, чем `--token BTC --token ETH --token SOL`
- Пользователи знают CSV-формат
- Можно комбинировать с `num_args`

---

## 🔥 Совет №18: subcommand_required — защита от пустого запуска

```rust
#[command(subcommand_required = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
// Если запустить без команды — clap выдаст help и завершится
```

### 🤔 Почему?
- Без этого флага `cargo run --` ничего не сделает
- С флагом: clap покажет help и сообщение об ошибке
- Пользователь сразу видит, какие команды доступны

---

## 📚 Что делать после этого файла

1. **Открой `01_hello_crypto.rs`** — самый простой пример
2. **Запусти**: `cargo run --example 01_hello_crypto -- --help`
3. **Потом** — `02_check_balance.rs` (добавляется валидация)
4. **Потом** — `03_send_transaction.rs` (субкоманды)
5. **Потом** — `04_crypto_wallet.rs` (все фичи вместе)
6. **Сделай** задание `exercise.rs`

Каждый пример содержит `// 💡 ADVICE:` комментарии прямо в коде.

> *Если что-то непонятно — просто спроси меня. Я объясню любую строку.*
