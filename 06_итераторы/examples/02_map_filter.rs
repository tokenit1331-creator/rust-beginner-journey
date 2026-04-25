// ============================================================
// 🟡 Шаг 2: map, filter, fold, take, skip
// Тема: Цепочки преобразований
// Контекст: Фильтрация транзакций и подсчёт сумм
// ============================================================

#[derive(Debug, Clone)]
struct Transaction {
    from: String,
    to: String,
    amount: f64,
    status: String, // "success", "pending", "failed"
}

fn main() {
    let txs = vec![
        Transaction { from: "0xABC".into(), to: "0xDEF".into(), amount: 0.5,  status: "success".into() },
        Transaction { from: "0xGHI".into(), to: "0xJKL".into(), amount: 2.0,  status: "success".into() },
        Transaction { from: "0xMNO".into(), to: "0xPQR".into(), amount: 0.01, status: "failed".into()  },
        Transaction { from: "0xSTU".into(), to: "0xVWX".into(), amount: 1.5,  status: "pending".into() },
        Transaction { from: "0xYZA".into(), to: "0xBCD".into(), amount: 3.0,  status: "success".into() },
        Transaction { from: "0xEFG".into(), to: "0xHIJ".into(), amount: 0.2,  status: "failed".into()  },
    ];

    // ============================================================
    // 1️⃣ filter() — отбираем элементы по условию
    //
    // 💡 ADVICE: filter() принимает closure, который возвращает
    // bool. true — оставляем, false — пропускаем.
    // filter() НЕ изменяет тип элемента, только убирает лишние.
    // ============================================================
    println!("=== 1. filter() — только успешные транзакции ===");

    let successful: Vec<&Transaction> = txs.iter()
        .filter(|tx| tx.status == "success")  // оставляем только success
        .collect();

    for tx in &successful {
        println!("  {} → {} | {} ETH ✅", tx.from, tx.to, tx.amount);
    }

    // ============================================================
    // 2️⃣ map() — преобразуем каждый элемент
    //
    // 💡 ADVICE: map() принимает closure, который преобразует
    // элемент из одного типа в другой. Часто используется
    // после filter() для извлечения конкретного поля.
    // ============================================================
    println!("\n=== 2. map() — извлекаем суммы ===");

    let amounts: Vec<f64> = txs.iter()
        .map(|tx| tx.amount)  // берём только поле amount
        .collect();

    println!("  Суммы транзакций: {:?}", amounts);

    // ============================================================
    // 3️⃣ filter() + map() = цепочка
    //
    // 💡 ADVICE: filter() сначала, map() потом!
    // Сначала отсеиваем, потом преобразуем — меньше работы.
    // Читается как: «из транзакций → берём success → берём суммы»
    // ============================================================
    println!("\n=== 3. filter() + map() — успешные суммы ===");

    let success_amounts: Vec<f64> = txs.iter()
        .filter(|tx| tx.status == "success")   // 1. отбираем
        .map(|tx| tx.amount)                    // 2. извлекаем
        .collect();

    println!("  Успешные: {:?}", success_amounts);

    // ============================================================
    // 4️⃣ fold() — сворачиваем в одно значение
    //
    // 💡 ADVICE: fold() — это reduce из других языков.
    // Начинаем с аккумулятора (0.0) и применяем функцию
    // к каждому элементу, обновляя аккумулятор.
    // ============================================================
    println!("\n=== 4. fold() — считаем общую сумму ===");

    let total: f64 = txs.iter()
        .map(|tx| tx.amount)
        .fold(0.0, |acc, amount| acc + amount);
    //        ^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^
    //    начальное  (аккумулятор, элемент) -> новый аккумулятор
    //    значение

    println!("  Общая сумма всех транзакций: {} ETH", total);

    // fold() можно использовать иначе — найти максимум:
    let max_amount: f64 = txs.iter()
        .map(|tx| tx.amount)
        .fold(0.0_f64, |max, amt| if amt > max { amt } else { max });

    println!("  Максимальная сумма: {} ETH", max_amount);

    // ============================================================
    // 5️⃣ take() — берём первые N элементов
    //
    // 💡 ADVICE: take() полезен для пагинации или
    // когда нужно показать только первые результаты.
    // ============================================================
    println!("\n=== 5. take() — первые 3 транзакции ===");

    let first_three: Vec<&Transaction> = txs.iter()
        .take(3)  // берём только 3 первых
        .collect();

    for tx in &first_three {
        println!("  {} → {} ({} ETH)", tx.from, tx.to, tx.amount);
    }

    // ============================================================
    // 6️⃣ skip() — пропускаем первые N элементов
    //
    // 💡 ADVICE: skip() + take() = пагинация (страницы).
    // skip(3).take(3) = страница 2 по 3 элемента.
    // ============================================================
    println!("\n=== 6. skip() + take() — пагинация ===");

    let page_2: Vec<&Transaction> = txs.iter()
        .skip(3)   // пропускаем первые 3
        .take(3)   // берём следующие 3
        .collect();

    println!("  Страница 2:");
    for tx in &page_2 {
        println!("    {} → {} ({} ETH)", tx.from, tx.to, tx.amount);
    }

    // ============================================================
    // 🧠 ИТОГ:
    // filter() — отбор (bool)
    // map()     — преобразование (T -> U)
    // fold()    — свёртка (T -> одно значение)
    // take()    — первые N
    // skip()    — пропустить N
    // Все можно комбинировать в цепочку!
    // ============================================================
    println!("\n✅ Итог: filter → map → fold — стандартная цепочка итераторов");
}
