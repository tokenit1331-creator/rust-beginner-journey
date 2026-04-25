# 💡 12 советов опытного разработчика по функциям в Rust

## 1. 🎯 Используй ранний return для ошибок

```rust
// ❌ Плохо: вложенный if-ад
fn process_tx(tx: &str) -> Result<Tx, String> {
    let parts: Vec<&str> = tx.split(',').collect();
    if parts.len() == 3 {
        Ok(Tx {})
    } else {
        Err("неверный формат".into())
    }
}

// ✅ Хорошо: ранний return, плоская структура
fn process_tx(tx: &str) -> Result<Tx, String> {
    let parts: Vec<&str> = tx.split(',').collect();
    if parts.len() != 3 {
        return Err("неверный формат".into());
    }
    Ok(Tx {})
}
```

## 2. 🔄 Используй комбинаторы вместо match

```rust
let result = do_something()
    .map(|v| format!("Успех: {}", v))
    .unwrap_or_else(|e| format!("Ошибка: {}", e));
```

## 3. 📛 Давай функциям понятные имена

```rust
// ❌ Плохо
fn proc(d: &[u8]) -> Vec<Tx> {}

// ✅ Хорошо
fn parse_transactions(raw_data: &[u8]) -> Vec<Transaction> {}
```

## 4. 📝 Документируй функции с ///

```rust
/// Парсит сырые данные транзакций
/// # Аргументы
/// * `raw` — массив байт
/// # Возвращает
/// Вектор транзакций
fn parse_transactions(raw: &[u8]) -> Vec<Transaction> {}
```

## 5. 🔨 Одна функция — одна ответственность

```rust
// ❌ Плохо: функция делает всё
fn process_file(path: &str) -> Result<(), String> {
    let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let txns: Vec<Tx> = raw.lines().filter_map(|l| { ... }).collect();
    let total: f64 = txns.iter().map(|t| t.amount).sum();
    println!("Всего: {}", total);
    Ok(())
}

// ✅ Хорошо: разделено на 4 функции
fn read_file(path: &str) -> Result<String, String> {}
fn parse_transactions(raw: &str) -> Vec<Transaction> {}
fn calculate_total(txns: &[Transaction]) -> f64 {}
fn print_summary(total: f64) {}
```

## 6. 🎯 Используй -> impl Trait для простоты

```rust
fn process_transactions(txns: Vec<Transaction>) -> impl Iterator<Item = f64> {
    txns.into_iter().map(|t| t.amount)
}
```

## 7. 🧩 Передавай замыкания как impl Fn

```rust
fn apply<F>(f: F, x: i32) -> i32
where F: Fn(i32) -> i32 {
    f(x)
}
```

## 8. 🧪 Тестируй функции отдельно

```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_tx() {
        let tx = parse_transaction("0xabc,1.5,BTC").unwrap();
        assert_eq!(tx.coin, "BTC");
    }
}
```

## 9. 🏁 Не бойся маленьких функций

```rust
fn is_large_transaction(tx: &Transaction) -> bool {
    tx.amount > 10_000.0
}
```

## 10. 📦 Используй Result для ошибок, не паникуй

```rust
fn get_first_tx(txns: Vec<Tx>) -> Result<Tx, String> {
    txns.into_iter().next().ok_or("Нет транзакций".into())
}
```

## 11. 🔗 Композиция функций

```rust
fn analyze_portfolio(path: &str) -> Result<PortfolioSummary, String> {
    let raw = read_file(path)?;
    let txns = parse_transactions(&raw)?;
    let valid = validate_transactions(&txns)?;
    Ok(calculate_summary(&valid))
}
```

## 12. 🎯 Используй дженерики разумно

```rust
// ❌ Избыточно
fn process<T: AsRef<str>>(data: T) {}
// ✅ Просто
fn process(data: &str) {}
```
