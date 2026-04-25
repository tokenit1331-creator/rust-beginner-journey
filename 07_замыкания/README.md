# 🦀 Замыкания (closures) в Rust: полное руководство для крипто‑CLI

Замыкания — это анонимные функции, которые могут захватывать переменные из окружающей области видимости. Они незаменимы при работе с итераторами, обработке ошибок, асинхронном коде и написании гибких API. В крипто‑CLI с их помощью удобно писать кастомные валидаторы, обработчики ошибок, трансформации данных.

---

## 1. Синтаксис замыканий

Замыкание записывается в виде `|параметры| тело`:

```rust
let add_one = |x: i32| -> i32 { x + 1 };
// тип параметров и возвращаемого значения часто выводятся автоматически
let add_one = |x| x + 1;
```

**Способы вызова:** `add_one(5)`.

Замыкания могут захватывать переменные из окружения тремя способами:
- `Fn` — неизменяемое заимствование (`&T`)
- `FnMut` — изменяемое заимствование (`&mut T`)
- `FnOnce` — перемещение владения (`T`)

Компилятор выводит нужный трейт автоматически в зависимости от того, как замыкание использует переменные.

---

## 2. Пять примеров использования замыканий в крипто‑CLI

### Пример 1. Валидация адреса через кастомный парсер `value_parser`

`clap` позволяет передать функцию-парсер. Замыкание удобно, если парсер короткий и не нуждается в отдельном именовании.

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(value_parser = |s: &str| -> Result<String, String> {
        if !s.starts_with("0x") || s.len() != 42 {
            return Err("адрес должен начинаться с 0x и иметь длину 42".into());
        }
        if !s[2..].chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("адрес содержит недопустимые символы".into());
        }
        Ok(s.to_lowercase())
    })]
    address: String,
}

fn main() {
    let args = Args::parse();
    println!("✅ Адрес принят: {}", args.address);
}
```

---

### Пример 2. Фильтрация транзакций по сумме (итераторы)

```rust
#[derive(Debug)]
struct Transaction {
    hash: String,
    amount: f64,
}

fn main() {
    let txs = vec![
        Transaction { hash: "0x1".into(), amount: 0.5 },
        Transaction { hash: "0x2".into(), amount: 10.0 },
        Transaction { hash: "0x3".into(), amount: 0.05 },
    ];
    let threshold = 1.0;

    let large_txs: Vec<_> = txs.iter()
        .filter(|tx| tx.amount > threshold)
        .collect();

    for tx in &large_txs {
        println!("Большая транзакция: {:?}", tx);
    }
}
```

---

### Пример 3. Обработка ошибок с `map_err` и `anyhow`

```rust
use anyhow::{Context, Result};

#[derive(Debug)]
struct ApiError {
    code: u16,
    message: String,
}

fn call_blockchain_api() -> Result<String, ApiError> {
    Err(ApiError { code: 429, message: "Too Many Requests".into() })
}

fn main() -> Result<()> {
    let response = call_blockchain_api()
        .map_err(|e| anyhow::anyhow!("Ошибка API (код {}): {}", e.code, e.message))
        .context("При получении баланса")?;
    println!("Ответ: {}", response);
    Ok(())
}
```

---

### Пример 4. Асинхронная подпись транзакции (замыкание внутри `spawn`)

```rust
use tokio::task;

async fn sign_transaction(tx_data: String, private_key: String) -> String {
    format!("signed_{}", tx_data)
}

#[tokio::main]
async fn main() {
    let private_key = "0xsecret".to_string();
    let tx = "send 0.1 ETH to 0xabc".to_string();

    let handle = task::spawn(async move {
        let signed = sign_transaction(tx, private_key).await;
        println!("Подписанная транзакция: {}", signed);
    });

    handle.await.unwrap();
}
```

---

### Пример 5. Замыкание как аргумент функции (пользовательский валидатор)

```rust
type Validator = Box<dyn Fn(&str) -> Result<(), String>>;

fn validate_address(addr: &str, validators: &[Validator]) -> Result<(), String> {
    for v in validators {
        v(addr)?;
    }
    Ok(())
}

fn main() {
    let hex_check = |s: &str| {
        if s[2..].chars().all(|c| c.is_ascii_hexdigit()) {
            Ok(())
        } else {
            Err("не hex".into())
        }
    };
    let length_check = |s: &str| {
        if s.len() == 42 { Ok(()) } else { Err("не та длина".into()) }
    };

    let addr = "0x1234567890123456789012345678901234567890";
    let validators: Vec<Validator> = vec![Box::new(hex_check), Box::new(length_check)];
    match validate_address(addr, &validators) {
        Ok(()) => println!("✅ Адрес корректен"),
        Err(e) => println!("❌ Ошибка: {}", e),
    }
}
```

---

## 3. Замыкания и производительность

Замыкания в Rust, как правило, **инлайнятся** (встраиваются) компилятором, поэтому их использование не приводит к накладным расходам по сравнению с ручным циклом. Однако при передаче замыкания как `dyn Fn` (trait‑объект) происходит виртуальный вызов – небольшая потеря производительности.

## 4. Советы опытного разработчика

1. Используйте замыкания в `clap` для коротких валидаторов.
2. В итераторах замыкания помогают избежать мусора.
3. Будьте внимательны с захватом переменных. Используйте `move` если замыкание должно жить дольше текущего фрейма.
4. Для обработки ошибок применяйте `map_err` и `and_then`.
5. Асинхронные замыкания (`async || {}`) – отличный выбор для параллельной обработки.
6. Профилируйте, если замыкание вызывается миллионы раз.
7. Используйте замыкания для кастомных стратегий retry.

## 5. Шпаргалка

| Задача | Решение с замыканием |
|--------|----------------------|
| Короткий парсер в `clap` | `#[arg(value_parser = \|s\| s.parse::<u64>())]` |
| Фильтрация вектора | `vec.iter().filter(\|x\| x > 5).collect()` |
| Преобразование ошибки | `result.map_err(\|e\| format!("ошибка {}", e))` |
| Асинхронная задача | `task::spawn(async move { do_something().await })` |
| Передача поведения в функцию | `fn retry<F>(f: F) where F: Fn() -> Result<...>` |

---

Замыкания — мощный и удобный инструмент. В крипто‑CLI они помогают писать лаконичный, понятный и эффективный код. 🚀