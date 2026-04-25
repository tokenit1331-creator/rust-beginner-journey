// ============================================================
// 🔴 УРОК: ПРОДВИНУТЫЕ ФУНКЦИИ
// ============================================================
//
// ДЛЯ НОВИЧКА: Здесь мы разбираем самые мощные возможности Rust:
// замыкания, обобщённые функции, комбинаторы (map/filter/fold).
//
// ЧТО ТЫ ВЫУЧИШЬ:
// 1. Замыкания — анонимные функции с захватом контекста
// 2. Fn vs FnMut vs FnOnce — три типа замыканий
// 3. Комбинаторы: map, filter, fold — без for-циклов
// 4. Обобщённые функции — одна для всех типов
// ============================================================

fn main() {
    println!("=== Продвинутые функции ===\n");
    println!("--- Демо 1: Простое замыкание ---");
    let fee_percent = 0.01;
    let calculate_fee = |amount: f64| -> f64 { amount * fee_percent };
    let fee = calculate_fee(1000.0);
    println!("Комиссия с $1000: ${}", fee);
    println!("\n--- Демо 2: Три типа замыканий ---");
    let rate = 1.2;
    let convert_fn = |amount: f64| amount * rate;
    println!("Fn: {}", convert_fn(100.0));
    println!("Fn: {} (второй раз норм)", convert_fn(200.0));
    let mut counter = 0;
    let mut count_fnmut = |item: &str| { counter += 1; println!("{}: {}", counter, item); };
    count_fnmut("BTC");
    count_fnmut("ETH");
    let data = vec!["tx1", "tx2", "tx3"];
    let consume = move || { println!("Обрабатываю: {:?}", data); };
    consume();
    println!("\n--- Демо 3: Комбинаторы ---");
    let transactions = vec![
        ("BTC", 100.0, "success"),
        ("ETH", 200.0, "success"),
        ("SOL", 50.0, "failed"),
        ("BTC", 300.0, "success"),
    ];
    let total_success: f64 = transactions
        .iter()
        .filter(|(_, _, status)| *status == "success")
        .map(|(_, amount, _)| amount)
        .sum();
    println!("💰 Успешных транзакций на сумму: ${}", total_success);
    println!("\n--- Демо 4: fold ---");
    let balances = vec![100.0, 200.0, 50.0, 300.0];
    let total: f64 = balances.iter().fold(0.0, |acc, val| acc + val);
    println!("Сумма балансов: ${}", total);
    println!("\n--- Демо 5: Обобщённые функции ---");
    fn find_max<T: PartialOrd>(items: &[T]) -> Option<&T> {
        let mut max = items.first()?;
        for item in items {
            if item > max { max = item; }
        }
        Some(max)
    }
    let numbers = vec![10, 5, 8, 20, 3];
    println!("Макс число: {:?}", find_max(&numbers));
    let words = vec!["banana", "apple", "cherry", "date"];
    println!("Макс слово: {:?}", find_max(&words));
    println!("\n--- Демо 6: fn pointer ---");
    fn add_one(x: i32) -> i32 { x + 1 }
    fn double(x: i32) -> i32 { x * 2 }
    fn apply(f: fn(i32) -> i32, x: i32) -> i32 { f(x) }
    println!("add_one(5) = {}", apply(add_one, 5));
    println!("double(5) = {}", apply(double, 5));
    println!("\n🎯 Замыкания, комбинаторы, обобщения — без for-циклов");
}
