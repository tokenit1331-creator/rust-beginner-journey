# 💡 12 советов по функциям от опытного разработчика

## 1️⃣ Одна функция — одно действие

Функция должна делать **только одно**. Если функция делает два дела — раздели.

```rust
// ❌ ПЛОХО: функция валидирует И логирует
fn process_tx(tx: &str) -> bool {
    println!("Обработка: {}", tx); // Логирование
    tx.len() >= 10                // Валидация
}

// ✅ ХОРОШО: разделяем
fn validate_tx(tx: &str) -> bool {
    tx.len() >= 10
}
fn log_tx(tx: &str) {
    println!("Обработка: {}", tx);
}
```

## 2️⃣ Ранний return для ошибок

Не вкладывай if-ы друг в друга. Проверяй ошибки и выходи.

```rust
// ❌ ПЛОХО: лесенка из if
fn process(order: &Order) -> Result<(), String> {
    if order.price > 0.0 {
        if order.amount > 0.0 {
            // логика...
            Ok(())
        } else {
            Err("amount <= 0".into())
        }
    } else {
        Err("price <= 0".into())
    }
}

// ✅ ХОРОШО: ранний return
fn process(order: &Order) -> Result<(), String> {
    if order.price <= 0.0 {
        return Err("price <= 0".into());
    }
    if order.amount <= 0.0 {
        return Err("amount <= 0".into());
    }
    // логика...
    Ok(())
}
```

## 3️⃣ Используй комбинаторы вместо for-циклов

map, filter, fold — читаются лучше, чем for с mut.

```rust
// ❌ ПЛОХО: for с mut
let mut sum = 0.0;
for tx in &txs {
    if tx.status == "success" {
        sum += tx.amount;
    }
}

// ✅ ХОРОШО: комбинаторы
let sum: f64 = txs.iter()
    .filter(|t| t.status == "success")
    .map(|t| t.amount)
    .sum();
```

## 4️⃣ Имена функций — глаголы

Функция — это действие. Имя должно говорить ЧТО делает.

```rust
fn validate() {}     // ✅ проверяет
fn calculate() {}    // ✅ вычисляет
fn process() {}      // ✅ обрабатывает
fn data() {}         // ❌ что делает? возвращает? печатает?
fn handler() {}      // ❌ что обрабатывает?
```

## 5️⃣ Параметры по ссылке (&), если владение не нужно

Не забирай владение, если функция только читает.

```rust
// ❌ ПЛОХО: забирает владение
fn print_balance(balance: Vec<f64>) {
    println!("{:?}", balance);
}

// ✅ ХОРОШО: только ссылка
fn print_balance(balance: &[f64]) {
    println!("{:?}", balance);
}
```

## 6️⃣ Тип возврата всегда пиши явно

Rust умеет выводить типы, но для функций — пиши всегда.

```rust
// ❌ ПЛОХО: неявный тип
fn process() -> _ { todo!() }

// ✅ ХОРОШО: явно
fn process() -> Result<f64, String> { todo!() }
```

## 7️⃣ Маленькие функции — это нормально

Функция на 3 строки — отлично. На 300 — переделывай.

## 8️⃣ Комментируй "почему", а не "что"

Код говорит "что", комментарии говорят "почему именно так".

```rust
// ❌ ПЛОХО: что и так видно
// прибавляем 1 к x
x + 1

// ✅ ХОРОШО: почему
// +1 потому что индекс считается с 0, а пользователь с 1
x + 1
```

## 9️⃣ Идиома: конструкторы new

Для создания структур используй `new()`.

```rust
impl Wallet {
    fn new(address: &str) -> Self {
        Self { address: address.to_string(), balance: 0.0 }
    }
}
```

## 🔟 Идиома: into() для конвертации

`.into()` превращает один тип в другой сам.

```rust
fn process(msg: String) {
    // msg уже String, .into() не нужен
}
fn process(msg: &str) {
    process(msg.to_string()); // явно превращаем
}
```

## 1️⃣1️⃣ Тестируй функции отдельно

Каждую функцию можно тестировать независимо.

```rust
#[test]
fn test_validate_address() {
    assert!(validate_address("0x1234..."));
    assert!(!validate_address("short"));
}
```

## 1️⃣2️⃣ Не бойся возвращать ошибки

Result — это не страшно. Это удобно.

```rust
fn parse_tx(raw: &str) -> Result<Tx, String> {
    // ... парсинг
    Ok(tx) // или Err("...")
}
```
