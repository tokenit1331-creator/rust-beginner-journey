# 11 — Итераторы и методы итераторов в Rust

## 🧠 Что такое итератор?

**Итератор** — это объект, который умеет перебирать элементы по одному.
Главный метод — `.next()`, который возвращает `Option<Item>`:
- `Some(item)` — есть ещё элемент
- `None` — перебор закончен

```rust
let v = vec![1, 2, 3];
let mut iter = v.iter();
assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), None);  // конец
```

## 📖 Структура урока

```
11_iterators/
├── examples/
│   ├── 01_basic_iterators.rs     # 🔵 iter, into_iter, iter_mut, for vs while
│   ├── 02_map_filter.rs          # 🟡 map, filter, fold, take, skip
│   ├── 03_chain_zip_flatmap.rs   # 🟠 chain, zip, flat_map, rev, step_by
│   ├── 04_collect_and_own.rs     # 🔴 collect, collect в HashMap, ownership
│   └── exercise.rs               # 🎯 Задание — крипто-портфель
├── tests/
│   └── iterator_tests.rs         # 12 тестов
├── src/main.rs                   # Навигатор
├── ADVICE.md                     # 17 советов
├── README.md
└── Cargo.toml
```

## 🚀 Как запускать

```bash
cd 11_iterators
cargo run --example 01_basic_iterators
cargo run --example 02_map_filter
cargo run --example 03_chain_zip_flatmap
cargo run --example 04_collect_and_own
cargo run --example exercise
cargo test
```

## 🎯 Что ты научишься

- ✅ Использовать `iter()`, `into_iter()`, `iter_mut()`
- ✅ Применять `map`, `filter`, `fold`, `take`, `skip`
- ✅ Склеивать итераторы через `chain`, `zip`, `flat_map`
- ✅ Собирать результаты в `Vec`, `HashMap`, `BTreeMap`
- ✅ Понимать ленивость и ownership в итераторах
