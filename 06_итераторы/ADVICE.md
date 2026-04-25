# 💡 Советы от опытного разработчика: Итераторы

## 1️⃣ Итераторы ленивы
```rust
let v = vec![1, 2, 3];
v.iter().map(|x| x + 1);  // ❌ НИЧЕГО не произошло!
// Надо:
let v2: Vec<_> = v.iter().map(|x| x + 1).collect();  // ✅
```
**Почему:** Это zero-cost abstraction — Rust не делает лишней работы.

## 2️⃣ `iter()` vs `into_iter()` vs `iter_mut()`
```rust
// iter() — ссылки (&T), можно читать после
for x in vec.iter() { }

// iter_mut() — мутабельные ссылки (&mut T)
for x in vec.iter_mut() { *x += 1; }

// into_iter() — владение (T), vec больше НЕДОСТУПЕН
for x in vec.into_iter() { }
```

## 3️⃣ `filter()` сначала, `map()` потом
```rust
// 👍 Лучше — меньше элементов обрабатывается
data.iter()
    .filter(|x| x.is_valid())
    .map(|x| x.transform())
    .collect()

// 👎 Хуже — сначала обработали все, потом отфильтровали
data.iter()
    .map(|x| x.transform())
    .filter(|x| x.is_valid())
    .collect()
```

## 4️⃣ `fold()` — замена циклу с аккумулятором
```rust
// Цикл
let mut sum = 0;
for x in &data { sum += x; }

// fold (функциональный стиль)
let sum = data.iter().fold(0, |acc, x| acc + x);
```

## 5️⃣ `zip()` останавливается на коротком
```rust
let a = [1, 2, 3];
let b = ['a', 'b'];
let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
// [(1, 'a'), (2, 'b')]  — '3' отброшен!
```

## 6️⃣ `flat_map()` для вложенных структур
```rust
let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
let flat: Vec<_> = nested.iter().flat_map(|x| x.iter()).collect();
// [1, 2, 3, 4, 5]
```

## 7️⃣ `chain()` — склеивание итераторов
```rust
let a = vec![1, 2];
let b = vec![3, 4];
let chained: Vec<_> = a.iter().chain(b.iter()).collect();
// [1, 2, 3, 4]
```

## 8️⃣ `rev()` и `step_by()`
```rust
let nums = vec![1, 2, 3, 4, 5, 6];
let rev: Vec<_> = nums.iter().rev().collect();  // [6,5,4,3,2,1]
let step: Vec<_> = nums.iter().step_by(2).collect();  // [1,3,5]
```

## 9️⃣ collect() знает, что собирать
```rust
let nums = vec![1, 2, 3];

// Явно указываем тип
let v: Vec<i32> = nums.iter().map(|x| x * 2).collect();
let set: HashSet<i32> = nums.iter().cloned().collect();
let map: HashMap<i32, i32> = nums.iter().map(|&x| (x, x * 10)).collect();
```

## 🔟 collect::<Result<_>>() — поймать первую ошибку
```rust
let results: Vec<Result<i32, _>> = vec![Ok(1), Ok(2), Err("fail")];
let collected: Result<Vec<i32>, _> = results.into_iter().collect();
// Err("fail") — остановится на первой ошибке
```

## 1️⃣1️⃣ Чейнинг читается слева направо
```rust
let result: Vec<_> = data.iter()
    .filter(|x| x.active)
    .map(|x| x.value)
    .filter(|&v| v > 10.0)
    .collect();
// Читай: берём данные → фильтр → преобразуем → ещё фильтр → собираем
```

## 1️⃣2️⃣ Тестируй итераторы
```rust
#[test]
fn test_map() {
    let v = vec![1, 2, 3];
    let result: Vec<_> = v.iter().map(|x| x * 2).collect();
    assert_eq!(result, vec![2, 4, 6]);
}
```

## 1️⃣3️⃣ `for` внутри — тоже итератор
Когда ты пишешь `for x in vec`, Rust вызывает `vec.into_iter()` неявно.

## 1️⃣4️⃣ Итераторы — zero-cost
Все итераторы компилируются в такой же машинный код, как ручной цикл. Rust выкидывает абстракции.

## 1️⃣5️⃣ Не все коллекции можно перебирать несколько раз
```rust
let v = vec![1, 2, 3];
for x in &v { }  // можно снова
for x in v.iter() { }  // можно снова
for x in v { }  // НЕЛЬЗЯ — v перемещён
```

## 1️⃣6️⃣ `enumerate()` — получить индекс
```rust
for (i, val) in vec.iter().enumerate() {
    println!("{}: {}", i, val);
}
```

## 1️⃣7️⃣ Всегда думай: «Могу ли я сделать это через итератор?»
Любой цикл, где ты преобразуешь, фильтруешь или сворачиваешь данные — кандидат на итератор. Это делает код короче и читаемее.
