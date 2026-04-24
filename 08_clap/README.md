# 🪙 Урок: clap — Парсинг аргументов командной строки

## Контекст: Крипто-кошелёк в терминале (Crypto CLI Wallet)

```
./crypto-wallet --address 0x123...abc --network ethereum balance
./crypto-wallet send --to 0x456...def --amount 0.5 --network polygon
./crypto-wallet history --days 7 --verbose
```

---

## 📋 Что ты узнаешь (Roadmap урока)

| Шаг | Файл | Что изучаем | Сложность |
|-----|------|-------------|-----------|
| 1 | `01_hello_crypto.rs` | Первый CLI: позиционные аргументы, `--help` | 🔵 Начало |
| 2 | `02_check_balance.rs` | Флаги, опции, значения по умолчанию | 🔵 Начало |
| 3 | `03_send_transaction.rs` | Субкоманды, валидация адресов | 🟡 Средний |
| 4 | `04_crypto_wallet.rs` | Все фичи: env, conflicts, count, subcommands | 🟠 Продвинутый |
| 5 | `exercise.rs` | Задание: создать свой крипто CLI | 🔴 Практика |

---

## 📖 Как читать этот урок

Каждый файл содержит:
1. **Код** — полный, рабочий пример
2. **Построчные комментарии** — каждая строка объяснена
3. **Блоки ПОЧЕМУ** — объяснение логики принятия решений
4. **Реальный контекст** — как это используется в реальных проектах

---

## 🚀 Быстрый старт

```bash
# 1. Клонируй репозиторий
git clone https://github.com/tokenit1331-creator/rust-beginner-journey.git
cd rust-beginner-journey/08_clap

# 2. Запусти каждый пример по очереди
cargo run --example 01_hello_crypto -- 0x123abc
cargo run --example 02_check_balance -- --address 0x123abc --network ethereum
cargo run --example 03_send_transaction -- send --to 0x456def --amount 0.5

# 3. Чтобы увидеть --help:
cargo run --example 04_crypto_wallet -- --help
```

---

## 🛠️ Где это применяется в реальном коде

| Проект | Описание |
|--------|----------|
| **solana-cli** | CLI для блокчейна Solana — использует clap с субкомандами |
| **ethers-rs** | Rust-библиотека для Ethereum — внутренний CLI |
| **cast** (foundry) | CLI-инструмент для взаимодействия с EVM-цепочками |
| **rust-bitcoin** | Утилиты для работы с Bitcoin |
| **cargo** (сам Rust) | Использует clap для парсинга команд |

---

## 📊 Сравнение: Derive vs Builder

| Критерий | Derive (мы используем) | Builder |
|----------|----------------------|---------|
| Код | 🟢 Минимум | 🔴 Много boilerplate |
| Читаемость | 🟢 Как структура данных | 🟡 Как цепочка вызовов |
| Ошибки компиляции | 🟢 clap проверяет типы | 🟡 Ошибки только в рантайме |
| Гибкость | 🟡 Достаточно для 95% задач | 🟢 Полный контроль |
| Для новичка | ✅ **Выбираем его** | ⚠️ Сложнее |

> **Почему мы выбрали Derive:**
> 1. Меньше кода = меньше ошибок
> 2. Компилятор проверяет типы аргументов на этапе сборки
> 3. Легче читать: вся структура CLI видна в одном месте
> 4. 95% реальных проектов используют Derive

---

## 📚 Полезные ссылки

- [Официальная документация clap](https://docs.rs/clap/latest/clap/)
- [Книга "The Rust Programming Language" — CLI глава](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [Rust by Example — CLI](https://doc.rust-lang.org/rust-by-example/std_misc/arg.html)
- [Solana CLI исходники](https://github.com/solana-labs/solana/tree/master/cli)
- [Foundry (cast) исходники](https://github.com/foundry-rs/foundry/tree/master/crates/cast)
