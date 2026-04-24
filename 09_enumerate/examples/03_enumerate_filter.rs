// ============================================================
// 🟠 ШАГ 3: enumerate() + filter() + map()
// ============================================================
//
// Что изучим:
//   ✅ filter ДО enumerate — новые индексы
//   ✅ filter ПОСЛЕ enumerate — исходные индексы
//   ✅ enumerate + filter + map в цепочке
//
// Контекст: Валидация крипто-транзакций

// Важно: порядок filter() и enumerate() МЕНЯЕТ индексы!
// filter ДО enumerate   → индексы пересчитываются (0, 1, 2...)
// filter ПОСЛЕ enumerate → индексы исходные

fn main() {
    #[derive(Debug)]
    struct Transaction {
        hash: String,
        amount: f64,
        valid: bool,
    }

    let txs = vec![
        Transaction { hash: "0xaaa".into(), amount: 0.5,  valid: true },
        Transaction { hash: "0xbbb".into(), amount: 0.0,  valid: false },
        Transaction { hash: "0xccc".into(), amount: 1.2,  valid: true },
        Transaction { hash: "0xddd".into(), amount: 0.0,  valid: false },
        Transaction { hash: "0xeee".into(), amount: 2.0,  valid: true },
    ];

    println!("=== ВСЕ ТРАНЗАКЦИИ ===");
    for (i, tx) in txs.iter().enumerate() {
        let status = if tx.valid { "✅" } else { "❌" };
        println!("  [{}] {} {} ETH — {}", i, status, tx.amount, tx.hash);
    }

    println!();
    println!("=== СЦЕНАРИЙ 1: filter ПОСЛЕ enumerate (исходные индексы) ===");

    // Сначала нумеруем (0..4), потом фильтруем
    let valid_with_original: Vec<(usize, &Transaction)> = txs.iter()
        .enumerate()
        .filter(|(_, tx)| tx.valid)
        .collect();

    println!("Валидные (исходные индексы):");
    for (i, tx) in &valid_with_original {
        println!("  [{}] {} ETH — {}", i, tx.amount, tx.hash);
    }
    // [0], [2], [4] — индексы из оригинального списка!

    println!();
    println!("=== СЦЕНАРИЙ 2: filter ДО enumerate (новые индексы) ===");

    // Сначала фильтруем, потом нумеруем
    let valid_with_new: Vec<(usize, &Transaction)> = txs.iter()
        .filter(|tx| tx.valid)
        .enumerate()
        .collect();

    println!("Валидные (новые индексы, без пропусков):");
    for (i, tx) in &valid_with_new {
        println!("  [{}] {} ETH — {}", i, tx.amount, tx.hash);
    }
    // [0], [1], [2] — нумерация с 0!

    println!();
    println!("=== СЦЕНАРИЙ 3: enumerate + filter + map ===");

    let valid_hashes: Vec<(usize, &str)> = txs.iter()
        .enumerate()
        .filter(|(_, tx)| tx.valid)
        .map(|(i, tx)| (i, tx.hash.as_str()))
        .collect();

    println!("Хеши валидных транзакций с номерами:");
    for (i, hash) in &valid_hashes {
        println!("  [{}] {}", i, hash);
    }

    println!();
    println!("=== СЦЕНАРИЙ 4: Фильтрация по условию ===");

    let large_txs: Vec<(usize, &Transaction)> = txs.iter()
        .enumerate()
        .filter(|(_, tx)| tx.amount > 1.0)
        .collect();

    println!("Транзакции больше 1 ETH:");
    for (i, tx) in &large_txs {
        println!("  [{}] {} ETH — {}", i, tx.amount, tx.hash);
    }

    println!();
    println!("=== БОНУС: Пагинация с skip + take ===");

    let page_size = 2;
    let page_number = 1;

    let page: Vec<_> = txs.iter()
        .enumerate()
        .skip(page_number * page_size)
        .take(page_size)
        .collect();

    println!("Страница {} (по {}):", page_number + 1, page_size);
    for (idx, tx) in &page {
        let status = if tx.valid { "✅" } else { "❌" };
        println!("  [{}] {} {} ETH", idx, status, tx.amount);
    }

    println!();
    println!("=== ИТОГ ШАГА 3 ===");
    println!("✅ Ты научился:");
    println!("   - Различать filter до enumerate и после");
    println!("   - Комбинировать enumerate + filter + map");
    println!("   - Использовать skip + take для пагинации");
}
