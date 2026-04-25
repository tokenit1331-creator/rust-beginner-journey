// ─── Продвинутые функции: замыкания, комбинаторы, generics ───

#[derive(Debug, Clone, PartialEq)]
enum TxStatus { Pending, Confirmed, Failed }

#[derive(Debug, Clone)]
struct Transaction {
    id: u32,
    from: String,
    to: String,
    amount: f64,
    coin: String,
    status: TxStatus,
}

fn demonstrate_closures() {
    println!("--- Замыкания ---");
    let add_one = |x: f64| -> f64 { x + 1.0 };
    println!("add_one(5.0) = {}", add_one(5.0));
    let fee_percent = 0.001;
    let apply_fee = |amount: f64| amount * (1.0 - fee_percent);
    println!("После комиссии с 1000: ${:.2}", apply_fee(1000.0));
    let mut counter = 0;
    let mut increment = || { counter += 1; counter };
    println!("Счётчик: {}", increment());
    println!("Счётчик: {}", increment());
}

fn demonstrate_combinators(txns: &[Transaction]) {
    println!("\n--- Комбинаторы ---");
    let confirmed: Vec<&Transaction> = txns.iter()
        .filter(|t| t.status == TxStatus::Confirmed).collect();
    println!("Успешных транзакций: {}", confirmed.len());
    let amounts: Vec<f64> = txns.iter().map(|t| t.amount).collect();
    println!("Суммы: {:?}", amounts);
    let total: f64 = txns.iter().map(|t| t.amount).fold(0.0, |acc, x| acc + x);
    println!("Общая сумма: ${:.2}", total);
}

fn find_first<T, F>(items: &[T], predicate: F) -> Option<&T>
where F: Fn(&T) -> bool {
    items.iter().find(|item| predicate(item))
}

fn demonstrate_generics(txns: &[Transaction]) {
    println!("\n--- Обобщённые функции ---");
    let btc_tx = find_first(txns, |t| t.coin == "BTC");
    match btc_tx {
        Some(tx) => println!("Первая BTC транзакция: ID {}", tx.id),
        None => println!("BTC транзакций нет"),
    }
}

fn main() {
    println!("=== Продвинутые функции ===\n");
    let txns = vec![
        Transaction { id: 1, from: "0xaaa".into(), to: "0xbbb".into(), amount: 1.5, coin: "BTC".into(), status: TxStatus::Confirmed },
        Transaction { id: 2, from: "0xccc".into(), to: "0xddd".into(), amount: 0.0, coin: "ETH".into(), status: TxStatus::Failed },
        Transaction { id: 3, from: "0xeee".into(), to: "0xfff".into(), amount: 3.2, coin: "SOL".into(), status: TxStatus::Confirmed },
        Transaction { id: 4, from: "0xggg".into(), to: "0xhhh".into(), amount: 5.0, coin: "BTC".into(), status: TxStatus::Pending },
        Transaction { id: 5, from: "0xiii".into(), to: "0xjjj".into(), amount: 2.0, coin: "ETH".into(), status: TxStatus::Confirmed },
    ];
    demonstrate_closures();
    demonstrate_combinators(&txns);
    demonstrate_generics(&txns);
}