// ============================================================
// 🔴 ШАГ 4: collect в HashMap с enumerate()
// ============================================================
//
// Что изучим:
//   ✅ collect::<HashMap<usize, &T>>()
//   ✅ collect в BTreeMap, Vec
//   ✅ Обратная карта (хеш → индекс)
//   ✅ Группировка по категориям
//
// Контекст: Крипто-кошелёк — быстрый доступ по индексу

use std::collections::HashMap;

fn main() {
    #[derive(Debug)]
    struct Transaction {
        hash: String,
        from: String,
        to: String,
        amount: f64,
        token: String,
    }

    let transactions = vec![
        Transaction { hash: "0xaaa".into(), from: "0x111".into(), to: "0x222".into(), amount: 1.5, token: "ETH".into() },
        Transaction { hash: "0xbbb".into(), from: "0x333".into(), to: "0x444".into(), amount: 0.8, token: "USDC".into() },
        Transaction { hash: "0xccc".into(), from: "0x555".into(), to: "0x666".into(), amount: 10.0, token: "SOL".into() },
        Transaction { hash: "0xddd".into(), from: "0x777".into(), to: "0x888".into(), amount: 0.0, token: "ETH".into() },
    ];

    println!("=== ПРИМЕР 1: HashMap (индекс → транзакция) ===");

    let tx_map: HashMap<usize, &Transaction> = transactions.iter()
        .enumerate()
        .collect();

    println!("Транзакция по индексу 2:");
    if let Some(tx) = tx_map.get(&2) {
        println!("  Хеш: {}, Сумма: {} {}", tx.hash, tx.amount, tx.token);
    }

    println!();
    println!("=== ПРИМЕР 2: HashMap с фильтром ===");

    let non_zero_map: HashMap<usize, &Transaction> = transactions.iter()
        .enumerate()
        .filter(|(_, tx)| tx.amount > 0.0)
        .collect();

    println!("Ненулевые транзакции по индексу:");
    for (i, tx) in &non_zero_map {
        println!("  [{}] {} {} {}", i, tx.amount, tx.token, tx.hash);
    }

    println!();
    println!("=== ПРИМЕР 3: Разные коллекции ===");

    let vec_result: Vec<(usize, &Transaction)> = transactions.iter().enumerate().collect();
    println!("Vec: {} элементов (порядок сохранён)", vec_result.len());

    let btree: std::collections::BTreeMap<usize, &Transaction> = transactions.iter().enumerate().collect();
    println!("BTreeMap: {} элементов (отсортировано)", btree.len());

    println!();
    println!("=== ПРИМЕР 4: Обратная карта (хеш → индекс) ===");

    let reverse_map: HashMap<&str, usize> = transactions.iter()
        .enumerate()
        .map(|(i, tx)| (tx.hash.as_str(), i))
        .collect();

    let search = "0xccc";
    if let Some(i) = reverse_map.get(search) {
        println!("Транзакция {} на индексе {}", search, i);
    }

    println!();
    println!("=== ПРИМЕР 5: Группировка по токенам ===");

    let mut grouped: std::collections::BTreeMap<&str, Vec<(usize, &Transaction)>> =
        std::collections::BTreeMap::new();

    for (i, tx) in transactions.iter().enumerate() {
        grouped
            .entry(tx.token.as_str())
            .or_default()
            .push((i, tx));
    }

    for (token, txs) in &grouped {
        println!("  {}:", token);
        for (i, tx) in txs {
            println!("    [{}] {} {}", i, tx.amount, tx.hash);
        }
    }

    println!();
    println!("=== ИТОГ ШАГА 4 ===");
    println!("✅ Ты научился:");
    println!("   - collect() в HashMap, BTreeMap, Vec");
    println!("   - Обратная карта (хеш → индекс)");
    println!("   - Группировка с entry().or_default()");
}
