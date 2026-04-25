# 🔢 Урок: `enumerate()` в Rust — от А до Я

**Контекст:** крипто CLI (транзакции, ордербуки, кошелёк)

**🔗 Репозиторий:** `rust-beginner-journey/09_enumerate`

---

## 📖 Что такое `enumerate()`?

```rust
let v = vec!["a", "b", "c"];
for (i, val) in v.iter().enumerate() {
    println!("{}: {}", i, val);
    // 0: a
    // 1: b
    // 2: c
}
```

**Суть:** `enumerate()` — это адаптер итератора, который добавляет счётчик к каждому элементу. Он возвращает итератор кортежей `(usize, &T)`.

---

## 🧠 Зачем нужен `enumerate()`?

| Ситуация | Без enumerate | С enumerate |
|----------|-------------|-------------|
| Вывести список с номерами | Считаешь индекс вручную | `for (i, tx) in txs.iter().enumerate()` |
| Найти элемент по номеру | `txs[i]` с проверкой границ | `txs.iter().enumerate().find()` |
| Отфильтровать + сохранить позицию | Доп. переменные | `enumerate().filter().collect()` |
| Построить карту: номер → значение | Цикл с insert | `enumerate().collect::<HashMap>()` |

---

## 📂 Структура урока

```
09_enumerate/
├── Cargo.toml                            # Зависимости (только std)
├── README.md                             # Этот файл
├── ADVICE.md                             # 15+ советов от опытного разработчика
├── src/main.rs                           # Точка входа
│
├── examples/
│   ├── 01_basic_enumerate.rs             # 🔵 Шаг 1 — Азы: индексы в for-цикле
│   ├── 02_enumerate_with_crypto.rs       # 🟡 Шаг 2 — Крипто-контекст: find, position
│   ├── 03_enumerate_filter.rs            # 🟠 Шаг 3 — enumerate + filter + map
│   ├── 04_enumerate_collect.rs           # 🔴 Шаг 4 — collect в HashMap
│   └── exercise.rs                       # 🎯 Задание
│
└── tests/
    └── enumerate_tests.rs                # ✅ Тесты
```

---

## 🚀 Как запускать

```bash
cd rust-beginner-journey/09_enumerate

cargo run --example 01_basic_enumerate
cargo run --example 02_enumerate_with_crypto
cargo run --example 03_enumerate_filter
cargo run --example 04_enumerate_collect
cargo run --example exercise
cargo test
```

---

## 🧠 Что ты изучишь

- Базовый `enumerate()` в цикле `for`
- `enumerate()` + `find()` для поиска по индексу
- `enumerate()` + `position()` / `rposition()`
- `enumerate()` + `filter()` — исходные vs новые индексы
- `enumerate()` + `map()` + `collect()`
- `collect::<HashMap<usize, &T>>()` — карта индекс→значение
- `skip()` + `take()` — пагинация
- `zip(0..)` — альтернатива для нестандартного счёта
- Zero-cost abstraction

---

## ✅ Хватит ли этого материала?

**Да, на 95% для повседневной работы.** После этого урока ты будешь использовать `enumerate()` вместо ручного счётчика, сможешь комбинировать с `filter`, `map`, `collect` и будешь знать, когда нужен `enumerate()`, а когда `position()`.

**Главное — практика.** Открывай `01_basic_enumerate.rs` и погнали! 🚀
