// ============================================================
// 🟡 УРОК: РЕАЛЬНАЯ КРИПТО-БИРЖА
// ============================================================
//
// ДЛЯ НОВИЧКА: Этот файл показывает, как функции помогают
// в реальной крипто-торговле. Мы построим Orderbook (книгу заказов),
// стратегию торговли и скоринг монет.
//
// ЧТО ТЫ ВЫУЧИШЬ:
// 1. Функции как строительные блоки большой системы
// 2. Композиция функций — одна вызывает другую
// 3. Передача функций в функции (колбэки)
// 4. Зачем делить код на маленькие функции
// ============================================================

#[derive(Debug)]
struct Order {
    price: f64,
    amount: f64,
}

fn is_price_valid(price: f64) -> bool {
    price > 0.0
}

fn is_amount_valid(amount: f64) -> bool {
    amount > 0.0 && amount <= 1000.0
}

fn is_valid_order(price: f64, amount: f64) -> bool {
    is_price_valid(price) && is_amount_valid(amount)
}

fn score_coin(volume_24h: f64, price_change: f64, holders: u32) -> f64 {
    let mut score = 0.0;
    if volume_24h > 1_000_000.0 { score += 50.0; }
    else if volume_24h > 100_000.0 { score += 30.0; }
    else { score += 10.0; }
    if price_change > 10.0 { score += 30.0; }
    else if price_change > 0.0 { score += 15.0; }
    if holders > 10_000 { score += 20.0; }
    else if holders > 1_000 { score += 10.0; }
    score
}

fn execute_trades(orders: &[Order], should_sell: impl Fn(&Order) -> bool) {
    for order in orders {
        if should_sell(order) {
            println!("💹 Продаём: цена ${:.2}, кол-во {}", order.price, order.amount);
        }
    }
}

fn sell_if_profitable(order: &Order) -> bool {
    order.price > 50_000.0 && order.amount >= 0.1
}

fn create_sell_filter(min_price: f64) -> impl Fn(&Order) -> bool {
    move |order: &Order| -> bool {
        order.price >= min_price && order.amount >= 0.1
    }
}

fn main() {
    println!("=== Крипто-Торговля ===\n");
    println!("--- Демо 1: Валидация ордера ---");
    let price = 100.0;
    let amount = 5.0;
    if is_valid_order(price, amount) {
        println!("✅ Ордер валиден: ${} x {}", price, amount);
    } else {
        println!("❌ Ордер невалиден");
    }
    println!("\n--- Демо 2: Скоринг монет ---");
    let btc_score = score_coin(50_000_000.0, 2.5, 50_000);
    let shitcoin_score = score_coin(5_000.0, -10.0, 100);
    println!("🏆 BTC счёт: {:.1}", btc_score);
    println!("💩 Shitcoin счёт: {:.1}", shitcoin_score);
    println!("\n--- Демо 3: Торговля с колбэком ---");
    let orders = vec![
        Order { price: 51000.0, amount: 0.5 },
        Order { price: 49000.0, amount: 1.0 },
        Order { price: 52000.0, amount: 0.05 },
        Order { price: 48000.0, amount: 2.0 },
    ];
    println!("Стратегия: sell_if_profitable:");
    execute_trades(&orders, sell_if_profitable);
    println!("\n--- Демо 4: Замыкание ---");
    let sell_high = |o: &Order| o.price > 50000.0;
    println!("Стратегия: только price > $50K:");
    execute_trades(&orders, sell_high);
    println!("\n--- Демо 5: Фабрика функций ---");
    let filter = create_sell_filter(50_000.0);
    println!("Стратегия: price >= $50K AND amount >= 0.1:");
    execute_trades(&orders, filter);
    println!("\n🎯 Функции — это СТРОИТЕЛЬНЫЕ БЛОКИ.");
    println!("Маленькие функции → легко читать, тестировать, менять.");
}
