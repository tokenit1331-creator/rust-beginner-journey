// ============================================================
// 🟡 ШАГ 2: enumerate() в крипто-контексте
// ============================================================
//
// Что изучим:
//   ✅ enumerate() + find() — поиск по индексу
//   ✅ enumerate() + position() — альтернатива
//   ✅ Реальный пример: orderbook криптобиржи
//
// Контекст: Поиск транзакции по номеру + orderbook

fn main() {
    let transactions = vec![
        ("send",    0.5,   "0xabc"),   // индекс 0
        ("receive", 1.2,   "0xdef"),   // индекс 1
        ("send",    2.0,   "0xghi"),   // индекс 2
        ("stake",   5.0,   "0xjkl"),   // индекс 3
        ("receive", 0.8,   "0xmno"),   // индекс 4
    ];

    println!("=== ПРИМЕР 1: Поиск транзакции по номеру ===");

    // Способ 1: enumerate() + find()
    let target_index = 2; // "третья" транзакция
    let third_tx = transactions.iter()
        .enumerate()
        .find(|(i, _)| *i == target_index);

    match third_tx {
        Some((_, (tx_type, amount, hash))) => {
            println!("Транзакция #{}:", target_index + 1);
            println!("  Тип: {}", tx_type);
            println!("  Сумма: {} ETH", amount);
            println!("  Хеш: {}", hash);
        }
        None => println!("Транзакция #{} не найдена!", target_index + 1),
    }

    println!();
    println!("=== ПРИМЕР 2: enumerate() + position() ===");

    // position() — только индекс первого совпадения
    let first_send = transactions.iter()
        .position(|(tx_type, _, _)| *tx_type == "send");

    match first_send {
        Some(i) => {
            let (_, amount, hash) = &transactions[i];
            println!("Первая send-транзакция на индексе {}:", i);
            println!("  Сумма: {} ETH, хеш: {}", amount, hash);
        }
        None => println!("Send-транзакции не найдены"),
    }

    // rposition() — с конца списка
    let last_send = transactions.iter()
        .rposition(|(tx_type, _, _)| *tx_type == "send");

    match last_send {
        Some(i) => {
            let (_, amount, hash) = &transactions[i];
            println!("Последняя send-транзакция на индексе {}:", i);
            println!("  Сумма: {} ETH, хеш: {}", amount, hash);
        }
        None => println!("Send-транзакции не найдены"),
    }

    println!();
    println!("=== ПРИМЕР 3: Реальный кейс — Orderbook ===");

    #[derive(Debug)]
    struct Order {
        price: f64,
        volume: f64,
        side: OrderSide,
    }

    #[derive(Debug, PartialEq)]
    enum OrderSide {
        Bid, // покупка
        Ask, // продажа
    }

    let orders = vec![
        Order { price: 100.0, volume: 1.5, side: OrderSide::Bid },
        Order { price: 99.5,  volume: 2.0, side: OrderSide::Bid },
        Order { price: 101.0, volume: 0.5, side: OrderSide::Ask },
        Order { price: 101.5, volume: 3.0, side: OrderSide::Ask },
        Order { price: 99.0,  volume: 1.0, side: OrderSide::Bid },
    ];

    println!("Orderbook (книга заявок):");
    println!("{:<4} {:<8} {:<8} {}", "#", "Цена", "Объём", "Сторона");
    println!("{}", "-".repeat(35));

    for (i, order) in orders.iter().enumerate() {
        let side_str = match order.side {
            OrderSide::Bid => "Покупка",
            OrderSide::Ask => "Продажа",
        };
        println!("{:<4} {:<8} {:<8} {}", i, order.price, order.volume, side_str);
    }

    // Поиск заявки по цене
    let target_price = 101.0;
    let found: Option<(usize, &Order)> = orders.iter()
        .enumerate()
        .find(|(_, order)| (order.price - target_price).abs() < f64::EPSILON);

    match found {
        Some((i, order)) => {
            println!("Заявка на цену {} на позиции #{}", target_price, i);
        }
        None => println!("Заявка не найдена"),
    }

    // Только Bid-заявки с номерами
    println!("\nЗаявки на покупку (Bid):");
    for (i, order) in orders.iter().enumerate() {
        if order.side == OrderSide::Bid {
            println!("  #{} — цена: {}, объём: {}", i, order.price, order.volume);
        }
    }

    println!();
    println!("=== ИТОГ ШАГА 2 ===");
    println!("✅ Ты научился:");
    println!("   - Искать элемент по индексу: enumerate() + find()");
    println!("   - Находить индекс: position() / rposition()");
    println!("   - Применять enumerate() к структурам (Orderbook)");
}
