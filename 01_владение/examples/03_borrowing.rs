// ┌─────────────────────────────────────────────────────────────────────┐
// │    03_borrowing.rs — ЗАИМСТВОВАНИЕ В РЕАЛЬНОМ КОДЕ                │
// │                                                                     │
// │ Запуск: cargo run --example 03_borrowing                            │
// │                                                                     │
// │ Фокус: &T, &mut T, правила, lifetimes, slices                      │
// └─────────────────────────────────────────────────────────────────────┘

// =====================================================================
// 📊 СТРУКТУРЫ
// =====================================================================

/// Ордер в книге ордеров (биржа)
#[derive(Debug)]
struct Order {
    price: f64,
    amount: f64,
    order_type: OrderType,
}

#[derive(Debug, PartialEq)]
enum OrderType {
    Buy,
    Sell,
}

/// Книга ордеров (Orderbook)
#[derive(Debug)]
struct Orderbook {
    pair: String,
    bids: Vec<Order>, // ордера на покупку
    asks: Vec<Order>, // ордера на продажу
}

// =====================================================================
// 🔵 ПРИМЕР 1: МНОЖЕСТВЕННЫЕ &T ССЫЛКИ
// =====================================================================

fn multiple_readers() {
    println!("\n=== 🔵 ПРИМЕР 1: МНОГО ЧИТАТЕЛЕЙ ===");

    let orderbook = Orderbook {
        pair: String::from("BTC/USDT"),
        bids: vec![
            Order { price: 50000.0, amount: 0.5, order_type: OrderType::Buy },
            Order { price: 49900.0, amount: 1.0, order_type: OrderType::Buy },
        ],
        asks: vec![
            Order { price: 50100.0, amount: 0.3, order_type: OrderType::Sell },
            Order { price: 50200.0, amount: 0.7, order_type: OrderType::Sell },
        ],
    };

    // ✅ МНОГО неизменяемых ссылок — полностью безопасно!
    let ref1 = &orderbook;
    let ref2 = &orderbook;
    let ref3 = &orderbook;

    println!("ref1: {}", ref1.pair);
    println!("ref2: {} ордеров", ref2.bids.len());
    println!("ref3: {} ордеров", ref3.asks.len());

    println!("✅ Все три ссылки работают одновременно!");
}

// =====================================================================
// 🟡 ПРИМЕР 2: ОДИН ПИСАТЕЛЬ (&mut T)
// =====================================================================

fn add_order(orderbook: &mut Orderbook, price: f64, amount: f64, order_type: OrderType) {
    let order = Order {
        price,
        amount,
        order_type,
    };

    match order.order_type {
        OrderType::Buy => orderbook.bids.push(order),
        OrderType::Sell => orderbook.asks.push(order),
    }

    println!("📋 Добавлен ордер: {} по {} ({} {})",
        if order_type == OrderType::Buy { "BUY" } else { "SELL" },
        price, amount, orderbook.pair);
}

fn single_writer() {
    println!("\n=== 🟡 ПРИМЕР 2: ОДИН ПИСАТЕЛЬ ===");

    let mut orderbook = Orderbook {
        pair: String::from("ETH/USDT"),
        bids: vec![],
        asks: vec![],
    };

    // Берём &mut ссылку
    let writer = &mut orderbook;

    // ✅ Можно добавлять ордера
    add_order(writer, 3000.0, 1.5, OrderType::Buy);
    add_order(writer, 3010.0, 2.0, OrderType::Sell);

    // ❌ Нельзя взять вторую &mut
    // let writer2 = &mut orderbook; // ОШИБКА!

    println!("✅ Один писатель — всё чётко!");
}

// =====================================================================
// 🟠 ПРИМЕР 3: ЧЕРЕДОВАНИЕ &T И &mut T
// =====================================================================

fn get_best_bid(orderbook: &Orderbook) -> Option<&Order> {
    orderbook.bids.iter().max_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
}

fn reader_writer_sequence() {
    println!("\n=== 🟠 ПРИМЕР 3: ЧИТАТЕЛЬ -> ПИСАТЕЛЬ ===");

    let mut orderbook = Orderbook {
        pair: String::from("SOL/USDT"),
        bids: vec![
            Order { price: 150.0, amount: 10.0, order_type: OrderType::Buy },
            Order { price: 148.0, amount: 5.0, order_type: OrderType::Buy },
        ],
        asks: vec![],
    };

    // ✅ Сначала читаем
    {
        let reader = &orderbook;
        if let Some(best) = get_best_bid(reader) {
            println!("📊 Лучшая цена покупки: {}", best.price);
        }
    } // reader умирает

    // ✅ Потом пишем (читатель уже не используется)
    {
        let writer = &mut orderbook;
        add_order(writer, 151.0, 8.0, OrderType::Buy);
    } // writer умирает

    // ✅ Снова читаем
    {
        let reader = &orderbook;
        if let Some(best) = get_best_bid(reader) {
            println!("📊 Новая лучшая цена: {}", best.price);
        }
    }

    println!("✅ Чтение и запись не пересекаются!");
}

// =====================================================================
// 🔴 ПРИМЕР 4: СРЕЗЫ (Slices)
// =====================================================================

/// Находит первые N лучших ордеров (срез)
fn top_orders(orders: &[Order], n: usize) -> &[Order] {
    let len = orders.len();
    if n >= len {
        &orders[..] // весь массив
    } else {
        &orders[..n] // первые n элементов
    }
}

fn slices_example() {
    println!("\n=== 🔴 ПРИМЕР 4: СРЕЗЫ (SLICES) ===");

    let orders = vec![
        Order { price: 100.0, amount: 1.0, order_type: OrderType::Buy },
        Order { price: 99.0, amount: 2.0, order_type: OrderType::Buy },
        Order { price: 98.0, amount: 3.0, order_type: OrderType::Buy },
        Order { price: 97.0, amount: 4.0, order_type: OrderType::Buy },
    ];

    // Берём срез — первые 2 ордера
    let top2 = top_orders(&orders, 2);
    println!("📊 Топ 2 ордера:");
    for order in top2 {
        println!("   {} по {} ({} шт)",
            if order.order_type == OrderType::Buy { "BUY" } else { "SELL" },
            order.price, order.amount);
    }

    // ⚠️ Срез — это ссылка! Подчиняется правилам заимствования
    println!("   Всего ордеров: {} (оригинал жив)", orders.len());
}

// =====================================================================
// 🟣 ПРИМЕР 5: LIFETIMES (Время жизни)
// =====================================================================

/// Находит лучшую цену из двух книг ордеров
/// 'a означает: "возвращаемая ссылка живёт не дольше, чем оба входных"
fn find_best_price<'a>(orderbook1: &'a Orderbook, orderbook2: &'a Orderbook) -> Option<&'a Order> {
    let best1 = orderbook1.bids.iter().max_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    let best2 = orderbook2.bids.iter().max_by(|a, b| a.price.partial_cmp(&b.price).unwrap());

    match (best1, best2) {
        (Some(o1), Some(o2)) => {
            if o1.price > o2.price { Some(o1) } else { Some(o2) }
        }
        (Some(o1), None) => Some(o1),
        (None, Some(o2)) => Some(o2),
        (None, None) => None,
    }
}

fn lifetimes_example() {
    println!("\n=== 🟣 ПРИМЕР 5: LIFETIMES (ВРЕМЯ ЖИЗНИ) ===");

    let orderbook1 = Orderbook {
        pair: String::from("BTC/USDT"),
        bids: vec![
            Order { price: 50000.0, amount: 1.0, order_type: OrderType::Buy },
        ],
        asks: vec![],
    };

    let orderbook2 = Orderbook {
        pair: String::from("BTC/USDC"),
        bids: vec![
            Order { price: 49900.0, amount: 2.0, order_type: OrderType::Buy },
        ],
        asks: vec![],
    };

    // Компилятор следит, что ссылка не переживёт входные данные
    let best = find_best_price(&orderbook1, &orderbook2);

    match best {
        Some(order) => println!("🏆 Лучшая цена: {} ({})", order.price, order.amount),
        None => println!("❌ Нет ордеров на покупку"),
    }

    println!("✅ Lifetime 'a защитил от висячей ссылки!");
}

// =====================================================================
// ⚠️ ПРИМЕР 6: ЧАСТЫЕ ОШИБКИ (закомментировано)
// =====================================================================

fn _common_mistakes() {
    // Ошибка 1: Ссылка на локальную переменную
    /*
    fn get_order() -> &Order {
        let order = Order { price: 100.0, amount: 1.0, order_type: OrderType::Buy };
        &order // ❌ order умрёт после return!
    }
    */

    // Ошибка 2: Move после заимствования
    /*
    let order = Order { price: 100.0, amount: 1.0, order_type: OrderType::Buy };
    let reference = &order;
    let _moved = order; // ❌ Нельзя move при активной ссылке!
    println!("{}", reference.price);
    */

    // Ошибка 3: &mut после & (без сужения области)
    /*
    let mut s = String::from("Hello");
    let r1 = &s;
    let r2 = &mut s; // ❌ Нельзя!
    println!("{}", r1);
    */

    // Как исправить ошибку 3:
    let mut s = String::from("Hello");
    let r1 = &s;
    println!("{}", r1); // r1 использована — можно "убить"
    let r2 = &mut s; // ✅ Теперь можно!
    r2.push_str(" World");
    println!("{}", r2);
}

// =====================================================================
// 🧪 ГЛАВНАЯ
// =====================================================================

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("     🔗 ЗАИМСТВОВАНИЕ В RUST — РЕАЛЬНЫЕ ПРИМЕРЫ");
    println!("═══════════════════════════════════════════════════════");

    multiple_readers();
    single_writer();
    reader_writer_sequence();
    slices_example();
    lifetimes_example();

    println!("\n═══════════════════════════════════════════════════════");
    println!("  ✅ Все примеры выполнены!");
    println!("═══════════════════════════════════════════════════════");
}
