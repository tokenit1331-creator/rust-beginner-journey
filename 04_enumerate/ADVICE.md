# 💡 15+ советов по `enumerate()` от опытного разработчика

---

## Совет №1: enumerate() возвращает (usize, &T), индексы с 0

```rust
let coins = vec!["BTC", "ETH", "SOL"];
for (i, coin) in coins.iter().enumerate() {
    // i: 0, 1, 2 (не 1, 2, 3!)
    println!("{}: {}", i, coin);
}
```

---

## Совет №2: Не путай enumerate() с position()

- `enumerate()` → нужны **все** индексы
- `position()` → нужен индекс **первого** совпадения
- `rposition()` → нужен индекс **последнего** совпадения

---

## Совет №3: filter() до enumerate() или после?

- **filter ПОСЛЕ enumerate** → исходные индексы (номера в原始 списке)
- **filter ДО enumerate** → новые индексы (нумерация с 0 после фильтрации)

В крипто-контексте чаще нужны **исходные** индексы (номер транзакции в блоке).

---

## Совет №4: collect() в HashMap — карта «индекс → значение»

```rust
let map: HashMap<usize, &Transaction> = txs.iter().enumerate().collect();
```

Доступ за O(1) по индексу.

---

## Совет №5: zip(0..) — альтернатива для нестандартного счёта

```rust
// счёт с 1
for (i, val) in items.iter().zip(1..) { ... }
// чётные индексы
for (i, val) in items.iter().zip((0..).step_by(2)) { ... }
```

---

## Совет №6: enumerate() + find() — найти элемент по номеру

```rust
let third = transactions.iter()
    .enumerate()
    .find(|(i, _)| *i == 2);
```

Альтернатива: `transactions.get(2)` — проще, но не даёт enumerate.

---

## Совет №7: Zero-cost abstraction

`enumerate()` — это ленивый адаптер. Он НЕ создаёт новый Vec. Ноль выделений памяти, ноль копирований.

```rust
// 🔴 ПЛОХО: собираем Vec
let indexed: Vec<_> = items.iter().enumerate().collect();
for (i, item) in indexed { ... }

// ✅ ХОРОШО: ленивый итератор
for (i, item) in items.iter().enumerate() { ... }
```

---

## Совет №8: skip() + take() для пагинации

```rust
let page: Vec<_> = txs.iter()
    .enumerate()
    .skip(10)  // пропустить первые 10
    .take(10)  // взять 10
    .collect();
```

---

## Совет №9: Мутация с iter_mut()

```rust
let mut amounts = vec![0.1, 0.5, 1.0];
for (i, amount) in amounts.iter_mut().enumerate() {
    if i % 2 == 0 {
        *amount *= 2.0;
    }
}
```

---

## Совет №10: Читаемость цепочек

Каждый метод на новой строке:

```rust
let result: Vec<_> = txs.iter()
    .enumerate()
    .filter(|(_, tx)| tx.amount > 0.1)
    .map(|(i, tx)| (i, tx.hash))
    .collect();
```

---

## Совет №11: for_each vs for

`for` — читаемее. `for_each` — стиль функционального программирования. Выбирай `for` по умолчанию.

---

## Совет №12: chars() vs bytes() для строк

```rust
// chars() — Unicode (правильно для русского)
for (i, c) in "привет".chars().enumerate() {
    print!("{i}:{c} "); // 0:п 1:р 2:и 3:в 4:е 5:т
}
// bytes() — сырые байты UTF-8
for (i, b) in "привет".bytes().enumerate() {
    print!("{i}:{b} "); // 0:208 1:191 2:209 3:128 ...
}
```

---

## Совет №13: Тестируй логику с enumerate

```rust
#[test]
fn test_filter_before() {
    let r: Vec<_> = items.iter()
        .filter(|s| **s == "send")
        .enumerate()
        .collect();
    assert_eq!(r, vec![(0, &"send"), (1, &"send")]);
}

#[test]
fn test_filter_after() {
    let r: Vec<_> = items.iter()
        .enumerate()
        .filter(|(_, s)| **s == "send")
        .collect();
    assert_eq!(r, vec![(0, &"send"), (2, &"send")]);
}
```

---

## Совет №14: rev() + enumerate() — обратный порядок

```rust
for (i, tx) in txs.iter().rev().enumerate() {
    // 0: самая новая, 1: следующая...
}
```

Применение: новые транзакции показывают первыми.

---

## Совет №15: Не оптимизируй заранее

`enumerate()` — zero-cost abstraction. Компилятор сделает код таким же эффективным, как ручной счётчик. Используй без страха.
