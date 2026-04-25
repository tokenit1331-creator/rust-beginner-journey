// ============================================================
// 🔴 Шаг 4: collect и ownership
// Тема: collect в разные структуры, borrow checker
// Контекст: сбор данных из транзакций в HashMap
// ============================================================

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Transaction {
    id: u32,
    from: String,
    to: String,
    amount: f64,
    status: String,
}

fn main() {
    let txs = vec![
        Transaction { id: 1, from: "0xABC".into(), to: "0xDEF".into(), amount: 0.5,  status: "success".into() },
        Transaction { id: 2, from: "0xGHI".into(), to: "0xJKL".into(), amount: 2.0,  status: "success".into() },
        Transaction { id: 3, from: "0xABC".into(), to: "0xMNO".into(), amount: 0.01, status: "failed".into()  },
        Transaction { id: 4, from: "0xPQR".into(), to: "0xABC".into(), amount: 1.5,  status: "success".into() },
        Transaction { id: 5, from: "0xSTU".into(), to: "0xVWX".into(), amount: 3.0,  status: "success".into() },
    ];

    // ============================================================
    // 1️⃣ collect() в Vec — самое простое
    //
    // 💡 ADVICE: collect() — один из самых мощных методов.
    // Он может собирать в Vec, HashMap, BTreeMap, HashSet,
    // Result, Option, String и другие.
    // Тип нужно указывать явно (или Rust выведет из контекста).
    // ============================================================
    println!("=== 1. collect() в Vec ===");

    let amounts: Vec<f64> = txs.iter()
        .map(|tx| tx.amount)
        .collect();  // собираем всё в Vec

    println!("  Суммы: {:?}", amounts);

    // ============================================================
    // 2️⃣ collect() в HashMap — ключ → значение
    //
    // 💡 ADVICE: HashMap собирается из кортежей (K, V).
    // Если ключи повторяются — последний перезаписывает.
    // ============================================================
    println!("\n=== 2. collect() в HashMap ===");

    // Собираем карту: id транзакции → её сумма
    // .map() возвращает кортежи (id, amount)
    // collect() видит (u32, f64) и собирает в HashMap<u32, f64>
    let id_to_amount: HashMap<u32, f64> = txs.iter()
        .map(|tx| (tx.id, tx.amount))  // создаём пары (ключ, значение)
        .collect();

    println!("  id → amount: {:?}", id_to_amount);
    println!("  Сумма транзакции #3: {} ETH", id_to_amount.get(&3).unwrap());

    // ============================================================
    // 3️⃣ collect() в HashMap — группировка по статусу
    //
    // 💡 ADVICE: Можно группировать данные по ключу.
    // Ключ — статус, значение — список сумм.
    // ============================================================
    println!("\n=== 3. Группировка по статусу ===");

    // Группируем: status → Vec<amount>
    let mut by_status: HashMap<String, Vec<f64>> = HashMap::new();

    for tx in &txs {
        // entry() — получает запись по ключу
        // or_default() — если нет, создаёт пустой Vec
        // push() — добавляет значение
        by_status.entry(tx.status.clone())
                 .or_default()
                 .push(tx.amount);
    }

    for (status, amounts) in &by_status {
        let total: f64 = amounts.iter().sum();
        println!("  {}: {:?} (всего: {} ETH)", status, amounts, total);
    }

    // ============================================================
    // 4️⃣ Ownership: когда итератор берёт владение
    //
    // 💡 ADVICE: iter() — забирает ссылки (borrow)
    // into_iter() — забирает владение (move)
    // Если тебе нужны сами данные, а не ссылки —
    // используй into_iter() и clone().
    // ============================================================
    println!("\n=== 4. Ownership: iter() vs into_iter() ===");

    // iter() — мы получаем ссылки (&Transaction)
    // Значит, txs остаётся жив
    let refs: Vec<&Transaction> = txs.iter()
        .filter(|tx| tx.status == "success")
        .collect();

    println!("  Ссылок на успешные: {}", refs.len());
    println!("  Оригинал всё ещё жив: {} транзакций", txs.len());

    // into_iter() — мы забираем владение
    // Создаём копию, чтобы txs остался
    let txs_copy = txs.clone();

    let owned: Vec<Transaction> = txs_copy.into_iter()
        .filter(|tx| tx.status == "success")
        .collect();

    println!("  Забранных (owned): {}", owned.len());
    // ❌ txs_copy больше недоступен
    // println!("{}", txs_copy.len());  // Ошибка!

    // ============================================================
    // 5️⃣ collect() с Result — поймать первую ошибку
    //
    // 💡 ADVICE: Если у тебя Vec<Result<T, E>>,
    // collect() может собрать Result<Vec<T>, E>.
    // Если все Ok — получишь Ok(Vec<T>).
    // Если хоть один Err — получишь первую ошибку.
    // ============================================================
    println!("\n=== 5. collect() с Result ===");

    let results = vec![Ok(1), Ok(2), Ok(3)];

    // collect<Result<Vec<_>, _>> — если все Ok, будет Ok(Vec)
    // Если есть Err — будет Err
    let collected: Result<Vec<i32>, &str> = results.into_iter().collect();

    match collected {
        Ok(nums) => println!("  Все успешно: {:?}", nums),
        Err(e) => println!("  Ошибка: {}", e),
    }

    // Пример с ошибкой
    let with_error = vec![Ok(1), Err("ошибка загрузки"), Ok(3)];
    let result: Result<Vec<i32>, &str> = with_error.into_iter().collect();

    match result {
        Ok(nums) => println!("  Все успешно: {:?}", nums),
        Err(e) => println!("  Ошибка: {}", e),
    }

    // ============================================================
    // 🧠 ИТОГ:
    // collect() в Vec          — базовый сбор
    // collect() в HashMap      — ключ → значение
    // entry().or_default()     — группировка
    // iter() vs into_iter()    — ссылки vs владение
    // collect<Result<_, _>>    — ошибки
    // ============================================================
    println!("\n✅ Итог: collect() умеет собирать в разные типы");
}
