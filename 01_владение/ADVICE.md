# 💡 Советы опытного разработчика: Ownership

## 📋 10 советов, которые сэкономят тебе часы дебага

---

## 1️⃣ Принимай `&T`, а не `T`, если функция только читает

```rust
// ❌ ПЛОХО: забирает владение
fn print_wallet(w: Wallet) {
    println!("{}", w.address);
}

// ✅ ХОРОШО: только читает
fn print_wallet(w: &Wallet) {
    println!("{}", w.address);
}
```

**Почему:** Вызывающий не теряет данные. Можно вызывать много раз.

---

## 2️⃣ Не злоупотребляй `.clone()`

```rust
// ❌ ПЛОХО: clone без причины
let data = get_large_data();
process(data.clone()); // зачем?
process(data); // можно было просто move

// ✅ ЛУЧШЕ: ссылка
let data = get_large_data();
process(&data);
process(&data);
```

**Почему:** `clone()` — глубокое копирование. Для больших структур это дорого.

---

## 3️⃣ Если ошибка «already borrowed» — сузь область видимости

```rust
// ❌ ОШИБКА
let mut s = String::from("Hello");
let r1 = &s;
let r2 = &mut s; // ❌
println!("{}", r1);

// ✅ ИСПРАВЛЕНИЕ
let mut s = String::from("Hello");
{
    let r1 = &s;
    println!("{}", r1);
} // r1 умерла
let r2 = &mut s; // ✅ теперь можно
```

**Почему:** Компилятор не даёт пересечься &T и &mut T. Помоги ему — разнеси по блокам.

---

## 4️⃣ Используй `&str` вместо `&String` в параметрах

```rust
// ❌ ПЛОХО: принимает только &String
fn greet(name: &String) {
    println!("Привет, {}", name);
}

// ✅ ХОРОШО: принимает любой строковый срез
fn greet(name: &str) {
    println!("Привет, {}", name);
}

// Теперь можно передавать:
let s = String::from("Alice");
greet(&s); // ✅ &String -> &str
let s2 = "Bob";
greet(s2); // ✅ &str -> &str
```

**Почему:** `&str` универсальнее — работает и с `String`, и со строковыми литералами.

---

## 5️⃣ Доверяй компилятору

Ошибка заимствования — **не баг, а защита**. Компилятор предотвратил:
- Гонку данных (data race)
- Висячую ссылку (dangling reference)
- Double free

Если код скомпилировался — он **безопасен**. Точка.

---

## 6️⃣ Не бойся lifetimes

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` — это просто «метка». Она говорит: «возвращаемая ссылка живёт не дольше входных».

В 90% случаев Rust сам выводит lifetimes. Явно писать нужно только когда:
- Функция принимает несколько ссылок и возвращает одну
- В структуре есть поля-ссылки

---

## 7️⃣ Используй срезы для части данных

```rust
// ❌ ПЛОХО: копируем данные
let words: Vec<&str> = text.split(' ').collect();

// ✅ ХОРОШО: срез — ссылка на оригинал
let first_word = &text[..5];
```

**Почему:** Срез — это просто указатель + длина. Ничего не выделяется.

---

## 8️⃣ `Into<T>` для гибких параметров

```rust
// Принимает и &str, и String, и Box<str>...
fn add_token<T: Into<String>>(wallet: &mut Wallet, token: T) {
    let token = token.into();
    wallet.tokens.insert(token, 0.0);
}

// Работает со всем:
add_token(&mut w, "BTC");           // &str
add_token(&mut w, String::from("ETH")); // String
```

---

## 9️⃣ Не храни ссылки в структурах без необходимости

```rust
// ❌ СЛОЖНО: нужно указывать lifetime
struct Wallet<'a> {
    address: &'a str,  // ссылка!
}

// ✅ ПРОСТО: владение
struct Wallet {
    address: String,  // владеет данными
}
```

**Почему:** `String` проще. Нет lifetime, нет проблем с заимствованием.

---

## 🔟 Тестируй владение через `#[derive(Debug)]`

```rust
#[derive(Debug)]
struct Transaction { ... }

let tx = Transaction { ... };
process(&tx);
println!("{:?}", tx); // ✅ Если tx жив — всё ок
```

---

## 📖 Запоминалка на каждый день

> **&T** = можно читать (много)
> **&mut T** = можно писать (один)
> **T** = забираю владение
>
> Если ошибка — сузь область видимости ссылок.
> Если не уверен — используй `&T`.
> Начинающим: сначала овладей ссылками, потом Clone, потом lifetimes.