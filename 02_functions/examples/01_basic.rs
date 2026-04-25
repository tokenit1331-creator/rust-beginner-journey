// ============================================================
// 🟢 УРОК: БАЗОВЫЕ ФУНКЦИИ В RUST
// ============================================================
//
// ДЛЯ НОВИЧКА: Этот файл — первое знакомство с функциями.
// Мы разберём 6 разных функций, каждая — для реальной задачи
// крипто-кошелька. Читай КАЖДУЮ функцию по порядку.
//
// ЧТО ТЫ ВЫУЧИШЬ:
// 1. Функция без параметров — просто блок кода
// 2. Функция с параметром — принимает данные
// 3. Функция с возвратом — отдаёт результат (bool)
// 4. Функция с Result — может вернуть ошибку
// 5. Функция со ссылками (&) — не забирает владение
// 6. Функция с &mut — может изменять данные
// 7. Функция внутри функции — вложенная область видимости
// ============================================================

fn print_title() {
    println!("=== Крипто-Кошелёк v1.0 ===");
}

fn print_balance(address: String) {
    println!("Баланс кошелька: {}", address);
}

fn has_sufficient_funds(balance: f64, amount: f64) -> bool {
    balance >= amount
}

fn validate_address(address: &str) -> Result<(), String> {
    if address.len() != 42 {
        return Err("Адрес должен быть 42 символа".to_string());
    }
    if !address.starts_with("0x") {
        return Err("Адрес должен начинаться с 0x".to_string());
    }
    Ok(())
}

fn calculate_portfolio_value(prices: &[f64], amounts: &[f64]) -> f64 {
    prices.iter().zip(amounts.iter()).map(|(p, a)| p * a).sum()
}

fn add_transaction(history: &mut Vec<String>, tx: String) {
    history.push(tx);
    println!("Транзакция добавлена. Всего: {}", history.len());
}

fn process_transaction() {
    fn log(message: &str) {
        println!("[LOG] {}", message);
    }
    log("Начинаем обработку...");
    log("Обработка завершена");
}

fn main() {
    println!("=== Базовые функции ===\n");
    print_title();
    let addr = String::from("0x1234567890abcdef1234567890abcdef12345678");
    print_balance(addr);
    let balance = 100.0;
    let amount = 50.0;
    if has_sufficient_funds(balance, amount) {
        println!("✅ Достаточно средств");
    }
    let test_addr = "0x1234567890abcdef1234567890abcdef12345678";
    match validate_address(test_addr) {
        Ok(_) => println!("✅ Адрес валиден"),
        Err(e) => println!("❌ {}", e),
    }
    let prices = vec![50000.0, 3000.0, 100.0];
    let amounts = vec![0.5, 2.0, 10.0];
    let total = calculate_portfolio_value(&prices, &amounts);
    println!("💰 Стоимость портфеля: ${:.2}", total);
    println!("  (prices доступны: {} элементов)", prices.len());
    let mut history: Vec<String> = Vec::new();
    add_transaction(&mut history, "Купил 0.1 BTC".to_string());
    add_transaction(&mut history, "Продал 2 ETH".to_string());
    println!("  История: {:?}", history);
    process_transaction();
}
