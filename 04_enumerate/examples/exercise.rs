// ============================================================
// 🎯 ЗАДАНИЕ: CLI-сканер крипто-транзакций с номерами
// ============================================================
//
// Используй всё, что изучил:
//   ✅ enumerate() в for-цикле
//   ✅ enumerate() + filter()
//   ✅ enumerate() + map() + collect()
//   ✅ collect в HashMap
//
// УСЛОВИЯ:
//   Список транзакций уже задан. Тебе нужно:
//   1. Вывести все транзакции с номерами
//   2. Вывести только успешные (с исходными индексами)
//   3. Создать HashMap (индекс → транзакция)
//   4. Создать обратную HashMap (хеш → индекс)
//   5. 🏆 БОНУС: Сгруппировать по типу (send/receive/stake)

#[derive(Debug, Clone)]
struct Tx {
    hash: String,
    tx_type: String,
    amount: f64,
    from: String,
    to: String,
    success: bool,
}

fn main() {
    let transactions = vec![
        Tx { hash: "0xa1b2".into(), tx_type: "send".into(), amount: 0.5, from: "0x111".into(), to: "0x222".into(), success: true },
        Tx { hash: "0xc3d4".into(), tx_type: "receive".into(), amount: 2.0, from: "0x333".into(), to: "0x444".into(), success: true },
        Tx { hash: "0xe5f6".into(), tx_type: "send".into(), amount: 1.5, from: "0x555".into(), to: "0x666".into(), success: false },
        Tx { hash: "0x7g8h".into(), tx_type: "stake".into(), amount: 10.0, from: "0x777".into(), to: "0x888".into(), success: true },
        Tx { hash: "0x9i0j".into(), tx_type: "receive".into(), amount: 0.0, from: "0x999".into(), to: "0xaaa".into(), success: false },
    ];

    // ==============================================================
    // ЗАДАНИЕ 1: Вывести ВСЕ транзакции с номерами
    // ==============================================================
    println!("=== ЗАДАНИЕ 1: Все транзакции ===");
    for (i, tx) in transactions.iter().enumerate() {
        let status = if tx.success { "✅" } else { "❌" };
        println!("  [{}] {} {} ETH → {} {}", i, tx.tx_type.to_uppercase(), tx.amount, tx.to, status);
    }

    // ==============================================================
    // ЗАДАНИЕ 2: Только успешные (с исходными индексами)
    // ==============================================================
    println!("\n=== ЗАДАНИЕ 2: Успешные транзакции ===");
    let successful: Vec<(usize, &Tx)> = transactions.iter()
        .enumerate()
        .filter(|(_, tx)| tx.success)
        .collect();

    for (i, tx) in &successful {
        println!("  [{}] {} {} ETH (хеш: {})", i, tx.tx_type, tx.amount, tx.hash);
    }

    // ==============================================================
    // ЗАДАНИЕ 3: HashMap индекс → транзакция
    // ==============================================================
    println!("\n=== ЗАДАНИЕ 3: HashMap (индекс → транзакция) ===");
    use std::collections::HashMap;
    let tx_map: HashMap<usize, &Tx> = transactions.iter()
        .enumerate()
        .collect();

    println!("Транзакция по индексу 2:");
    if let Some(tx) = tx_map.get(&2) {
        println!("  {} {} ETH (success: {})", tx.tx_type, tx.amount, tx.success);
    }

    // ==============================================================
    // ЗАДАНИЕ 4: Обратная HashMap (хеш → индекс)
    // ==============================================================
    println!("\n=== ЗАДАНИЕ 4: Обратная HashMap (хеш → индекс) ===");
    let reverse_map: HashMap<&str, usize> = transactions.iter()
        .enumerate()
        .map(|(i, tx)| (tx.hash.as_str(), i))
        .collect();

    let search = "0xe5f6";
    match reverse_map.get(search) {
        Some(i) => println!("Транзакция {} на индексе {}", search, i),
        None => println!("Транзакция {} не найдена", search),
    }

    // ==============================================================
    // 🏆 БОНУС: Группировка по типу
    // ==============================================================
    println!("\n=== 🏆 БОНУС: Группировка по типу ===");
    let mut grouped: std::collections::BTreeMap<&str, Vec<(usize, &Tx)>> =
        std::collections::BTreeMap::new();

    for (i, tx) in transactions.iter().enumerate() {
        grouped
            .entry(tx.tx_type.as_str())
            .or_default()
            .push((i, tx));
    }

    for (tx_type, txs) in &grouped {
        println!("  {} ({} шт.):", tx_type.to_uppercase(), txs.len());
        for (i, tx) in txs {
            let status = if tx.success { "✅" } else { "❌" };
            println!("    [{}] {} ETH {} {}", i, tx.amount, tx.to, status);
        }
    }

    println!("\n=== 🎉 ГОТОВО! ===");
    println!("Запусти `cargo test` чтобы проверить тесты!");
}
