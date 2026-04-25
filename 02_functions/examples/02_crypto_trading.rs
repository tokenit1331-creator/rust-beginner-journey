// ─── Реальный кейс: Крипто-биржа ───

#[derive(Debug, Clone)]
struct Coin {
    symbol: String,
    price: f64,
    volume_24h: f64,
    change_24h: f64,
}

struct Strategy {
    name: String,
    should_sell: Box<dyn Fn(&Coin) -> bool>,
    should_buy: Box<dyn Fn(&Coin) -> bool>,
}

fn score_coin(coin: &Coin) -> u32 {
    let mut score = 50u32;
    if coin.volume_24h > 1_000_000.0 { score += 20; }
    if coin.change_24h > 5.0 { score += 20; }
    else if coin.change_24h > 0.0 { score += 10; }
    if coin.change_24h < -5.0 { score = score.saturating_sub(20); }
    else if coin.change_24h < 0.0 { score = score.saturating_sub(10); }
    score
}

fn filter_coins<F>(coins: &[Coin], predicate: F) -> Vec<&Coin>
where F: Fn(&Coin) -> bool {
    coins.iter().filter(|c| predicate(c)).collect()
}

fn apply_strategy<'a>(coins: &'a [Coin], strategy: &Strategy) -> Vec<&'a Coin> {
    coins.iter()
        .filter(|c| (strategy.should_buy)(c) || (strategy.should_sell)(c))
        .collect()
}

fn main() {
    println!("=== Крипто-Биржа: Функции и Стратегии ===\n");
    
    let coins = vec![
        Coin { symbol: "BTC".into(), price: 50000.0, volume_24h: 28_000_000.0, change_24h: 2.5 },
        Coin { symbol: "ETH".into(), price: 3000.0, volume_24h: 15_000_000.0, change_24h: -1.2 },
        Coin { symbol: "SOL".into(), price: 100.0, volume_24h: 5_000_000.0, change_24h: 8.7 },
        Coin { symbol: "DOGE".into(), price: 0.05, volume_24h: 500_000.0, change_24h: -3.5 },
    ];
    
    println!("🔍 Монеты с объёмом > 1M:");
    let high_volume = filter_coins(&coins, |c| c.volume_24h > 1_000_000.0);
    for c in &high_volume {
        println!("   {} — ${:.2}", c.symbol, c.price);
    }
    
    println!("\n📊 Скоринг монет:");
    for coin in &coins {
        println!("   {}: {} баллов", coin.symbol, score_coin(coin));
    }
    
    let trend_strategy = Strategy {
        name: "Трендовая".to_string(),
        should_sell: Box::new(|c: &Coin| c.change_24h > 5.0),
        should_buy: Box::new(|c: &Coin| c.change_24h < -5.0),
    };
    
    println!("\n🎯 Стратегия: {}", trend_strategy.name);
    let actions = apply_strategy(&coins, &trend_strategy);
    for c in &actions {
        println!("   {} (изменение: {}%)", c.symbol, c.change_24h);
    }
}