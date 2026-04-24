// ============================================================
// 🔵 ШАГ 1: Базовый enumerate() — азы
// ============================================================
//
// Что изучим:
//   ✅ enumerate() в цикле for
//   ✅ Распаковка кортежа (i, val)
//   ✅ Индексация с 0
//   ✅ iter() vs iter_mut()
//
// Контекст: Список криптовалют в кошельке

fn main() {
    let coins = vec!["Bitcoin (BTC)", "Ethereum (ETH)", "Solana (SOL)", "Polygon (MATIC)"];

    println!("=== ПРИМЕР 1: Базовый enumerate() ===");
    
    // 🔴 БЕЗ enumerate — ручной счётчик
    let mut i = 0;
    for coin in &coins {
        println!("[{}] {}", i, coin);
        i += 1; // Легко забыть!
    }

    println!();

    // 🟢 С enumerate — правильно и безопасно
    // coins.iter() даёт итератор по ссылкам
    // .enumerate() добавляет счётчик (usize)
    // Результат: (0, &"Bitcoin"), (1, &"Ethereum"), ...
    for (i, coin) in coins.iter().enumerate() {
        println!("[{}] {}", i, coin);
    }

    println!();
    println!("=== ПРИМЕР 2: Демонстрация типов ===");

    let numbers = vec![42, 100, 255];
    for (i, num) in numbers.iter().enumerate() {
        let i_type = std::any::type_name_of_val(&i);
        let num_type = std::any::type_name_of_val(num);
        println!("(i: {}) имеет тип `{}`, значение `{}` имеет тип `{}`", i, i_type, num, num_type);
    }

    println!();
    println!("=== ПРИМЕР 3: iter_mut() — изменение значений ===");

    let mut balances = vec![1.5, 0.8, 10.2, 3.0]; // в ETH
    for (i, balance) in balances.iter_mut().enumerate() {
        *balance = (*balance * 10.0).round() / 10.0;
        println!("[{}] Баланс после округления: {} ETH", i, balance);
    }

    println!();
    println!("=== ПРИМЕР 4: enums + enumerate ===");

    #[derive(Debug)]
    enum TransactionType {
        Send(String),
        Receive(String),
        Stake(u64),
    }

    let activities = vec![
        TransactionType::Send("0xabc".into()),
        TransactionType::Receive("0xdef".into()),
        TransactionType::Stake(100),
        TransactionType::Send("0xghi".into()),
    ];

    for (i, activity) in activities.iter().enumerate() {
        print!("[{}] ", i);
        match activity {
            TransactionType::Send(to) => println!("Отправлено на {}", to),
            TransactionType::Receive(from) => println!("Получено от {}", from),
            TransactionType::Stake(amount) => println!("Застейкано {} SOL", amount),
        }
    }

    println!();
    println!("=== ИТОГ ШАГА 1 ===");
    println!("✅ Ты научился:");
    println!("   - .iter().enumerate() в цикле for");
    println!("   - Распаковывать кортеж (i, val)");
    println!("   - iter() vs iter_mut()");
    println!("   - enumerate с enum-ами");
}
