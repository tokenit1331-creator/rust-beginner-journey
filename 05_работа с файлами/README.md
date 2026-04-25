# 📂 Урок 10: Работа с файлами в Rust

## 🧠 Суть (на пальцах)

Файлы — это способ сохранить данные между запусками программы.
Без файлов всё, что ты сделал, исчезает при выходе.
С файлами — ты можешь сохранить ключи, конфиги, историю транзакций.

## 📖 Структура урока

```
10_file_io/
├── Cargo.toml                       # Зависимости
├── README.md                        # Шпаргалка
├── ADVICE.md                        # 15+ советов от опытного разработчика
├── src/main.rs                      # Навигатор по примерам
├── examples/
│   ├── 01_write_read_text.rs        # 🔵 Шаг 1 — Запись/чтение текста
│   ├── 02_json_wallet.rs            # 🟡 Шаг 2 — JSON + Serde
│   ├── 03_private_key_binary.rs     # 🟠 Шаг 3 — Бинарные ключи + append
│   ├── 04_csv_transactions.rs       # 🔴 Шаг 4 — CSV + BufReader
│   └── exercise.rs                  # 🎯 Задание
└── tests/
    └── file_io_tests.rs             # ✅ 12 тестов
```

## 🚀 Как запускать

```bash
cd rust-beginner-journey/10_file_io

# По порядку — от простого к сложному
cargo run --example 01_write_read_text
cargo run --example 02_json_wallet
cargo run --example 03_private_key_binary
cargo run --example 04_csv_transactions
cargo run --example exercise

# Тесты
cargo test
```

## 📊 Что ты научишься делать

| Что | Пример |
|-----|--------|
| Записать текст в файл | `fs::write()` |
| Прочитать текст из файла | `fs::read_to_string()` |
| Добавить в конец файла | `OpenOptions::new().append(true)` |
| Сохранить структуру в JSON | `serde_json::to_string_pretty()` |
| Прочитать JSON в структуру | `serde_json::from_str()` |
| Записать бинарные данные | `write_all()` |
| Разобрать CSV строку за строкой | `BufReader` + `.split()` |
| Обработать ошибки I/O | `?` оператор |

## 🤔 Этого хватит?

**Да, чтобы начать.** После этого урока ты сможешь:
- Сохранять/загружать конфиги кошелька (JSON)
- Работать с приватными ключами (бинарные файлы)
- Парсить историю транзакций (CSV)
- Правильно выбирать метод для задачи
- Обрабатывать ошибки I/O

## 💡 Быстрая шпаргалка

```rust
use std::fs;
use std::io::Write;

// 1. Самый простой способ — целиком
fs::write("file.txt", "привет")?;
let text = fs::read_to_string("file.txt")?;

// 2. Для JSON
let json = serde_json::to_string_pretty(&my_struct)?;
fs::write("config.json", &json)?;

// 3. Для добавления в конец
let mut file = std::fs::OpenOptions::new()
    .append(true)
    .create(true)
    .open("log.txt")?;
writeln!(file, "новая запись")?;

// 4. Для чтения больших файлов построчно
let reader = std::io::BufReader::new(std::fs::File::open("data.csv")?);
for line in reader.lines() {
    let line = line?;
    // обрабатываем строку
}
```

## 🔗 Ссылки

- [The Rust Programming Language — Chapter 12: I/O](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [std::fs документация](https://doc.rust-lang.org/std/fs/)
- [Serde документация](https://serde.rs/)
