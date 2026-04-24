# 🚀 Глубокое погружение в clap

**clap** (Command Line Argument Parser) — самая популярная библиотека Rust для парсинга CLI-аргументов. Используется в 50 000+ проектах.

## 📦 Установка

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
```

### Полная установка (все фичи)

```toml
clap = { version = "4.5", features = [
    "derive",      # макросы Parser, Subcommand, ValueEnum
    "env",         # чтение из переменных окружения
    "string",      # доп. методы для String
    "unicode",     # поддержка Unicode в help
    "wrap_help",   # перенос строк в help
    "suggestions", # предложения при ошибке ("может вы имели в виду --count?")
    "color",       # цветной вывод help
] }
```

## 🗺️ Структура проекта

```
08_clap/
├── Cargo.toml                                 # зависимости
├── src/
│   └── main.rs                                # [1] Базовый пример (все базовые фичи)
├── examples/
│   ├── 02_builder_pattern.rs                  # [2] Builder Pattern (без макросов)
│   ├── 03_subcommands.rs                      # [3] Субкоманды (как git commit)
│   ├── 04_validation.rs                       # [4] Валидация значений
│   ├── 05_conflicts.rs                        # [5] Зависимости и конфликты
│   ├── 06_advanced.rs                         # [6] Продвинутые фичи
│   └── 07_completion.rs                       # [7] Автодополнение для shell
├── README.md                                  # этот файл
└── exercise.md                                # задание для закрепления
```

## 📖 Как запускать

```bash
# Основной пример
cargo run -- --help
cargo run -- Артём -n 3 --formal --language ru -i код -i спорт

# Примеры по номерам
cargo run --example 02_builder_pattern -- --name Артём -v
cargo run --example 03_subcommands -- add "Купить хлеб" -p high
cargo run --example 04_validation -- --age 25 --weight 80 --mode fast
cargo run --example 05_conflicts -- --ship --insured --value 1000
API_KEY=mykey123 cargo run --example 06_advanced -- --api-key mykey -vv --tags rust,cli
cargo run --example 07_completion -- --completion bash
```

## 🧩 Два подхода: таблица сравнения

| Возможность              | Derive 🪄          | Builder 🔨          |
|-------------------------|-------------------|--------------------|
| Кода писать             | **Очень мало**     | Много              |
| Читаемость              | **Отличная**       | Средняя            |
| Валидация на компиляции | ✅                 | ❌ (runtime)        |
| Субкоманды              | ✅ (enum)          | ✅ (Command)        |
| Динамические аргументы  | ❌ (только статика)| ✅                 |
| Когда использовать      | **95% случаев**    | Сложные CLI        |

## 🎯 Шпаргалка (cheat sheet)

```rust
// === DERIVE АТРИБУТЫ ===

// Для структуры (Command)
#[command(name = "myapp")]         // имя программы
#[command(version = "1.0")]        // версия
#[command(author = "Я")]           // автор
#[command(about = "Описание")]     // краткое описание
#[command(long_about = "...")]    // полное описание
#[command(after_help = "...")]    // текст после help
#[command(before_help = "...")]   // текст перед help

// Для полей (Arg)
#[arg(short)]                      // -f (авто: первая буква)
#[arg(short = 'n')]                // -n (явно)
#[arg(long)]                       // --field-name
#[arg(long = "my-flag")]           // --my-flag (явно)
#[arg(default_value_t = 1)]        // значение по умолчанию
#[arg(default_value = "text")]     // строковое значение по умолчанию
#[arg(required = true)]            // обязательный
#[arg(required = false)]           // необязательный
#[arg(value_parser = ...)]        // кастомный парсер
#[arg(hide = true)]                // скрыть из help
#[arg(env = "VAR_NAME")]           // читать из переменной окружения
#[arg(conflicts_with = "other")]   // конфликт с другим
#[arg(requires = "other")]         // требует другой
#[arg(num_args = 1..=3)]           // количество значений
#[arg(action = clap::ArgAction::Count)]  // подсчёт повторов
#[arg(value_delimiter = ',')]      // разделитель значений
```

## 💡 Частые ошибки и их решения

| Ошибка | Причина | Решение |
|--------|---------|---------|
| `error: ... was used, but none was expected` | Передан неизвестный аргумент | Проверь название флага |
| `error: ... requires a value but none was supplied` | Флаг без значения | Добавь значение: `--flag value` |
| `error: ... could not be parsed` | Неправильный тип | Проверь формат (для u32 нужно число) |
| `error: ... is not supported` | Нет фичи derive | Добавь `features = ["derive"]` |
| `error: Found argument '...' which wasn't expected` | Опечатка в имени флага | clap предложит исправление (с фичей suggestions) |

## 🔗 Полезные ссылки

- [Официальная документация clap](https://docs.rs/clap)
- [The Rust CLI Book](https://rust-cli.github.io/book/)
- [clap examples на GitHub](https://github.com/clap-rs/clap/tree/master/examples)
- [Awesome Rust CLI](https://github.com/rust-cli/awesome-rust-cli)
