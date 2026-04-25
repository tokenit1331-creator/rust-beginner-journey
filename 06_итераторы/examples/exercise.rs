// ============================================================
// 🎯 Задание: Анализ крипто-портфеля через итераторы
//
// 🔥 Твоя задача — дописать 5 функций.
// Каждая функция использует методы итераторов.
//
// 💡 ADVICE: Читай сигнатуру функции — она подсказывает,
// какой итератор использовать.
// ============================================================

use std::collections::HashMap;

/// Структура монеты
#[derive(Debug, Clone)]
struct Coin {
    name: String,
    symbol: String,
    amount: f64,
    price_usd: f64,
    active: bool,
}

/// Портфель инвестора
struct Portfolio {
    coins: Vec<Coin>,
}

impl Portfolio {
    /// Создаём тестовый портфель
    fn new() -> Self {
        Self {
            coins: vec![
                Coin { name: "Bitcoin".into(),  symbol: "BTC".into(),  amount: 0.5,  price_usd: 67000.0, active: true  },
                Coin { name: "Ethereum".into(), symbol: "ETH".into(),  amount: 5.0,  price_usd: 3400.0,  active: true  },
                Coin { name: "Solana".into(),   symbol: "SOL".into(),  amount: 20.0, price_usd: 150.0,   active: true  },
                Coin { name: "Cardano".into(),  symbol: "ADA".into(),  amount: 100.0,price_usd: 0.45,    active: false },
                Coin { name: "Polkadot".into(), symbol: "DOT".into(),  amount: 50.0, price_usd: 7.0,     active: true  },
            ],
        }
    }

    // ============================================================
    // 🎯 Задание 1: Получить символы всех активных монет
    //
    // Должна вернуть: Vec<String> с символами (BTC, ETH, SOL, DOT)
    // Подсказка: filter + map + collect
    // ============================================================
    fn active_symbols(&self) -> Vec<String> {
        // 🟢 Твой код здесь
        // self.coins.iter()
        //     .filter(|c| ...)
        //     .map(|c| ...)
        //     .collect()
        todo!("Задание 1: отфильтруй активные и верни их символы")
    }

    // ============================================================
    // 🎯 Задание 2: Общая стоимость портфеля (только активных)
    //
    // Стоимость = сумма amount * price_usd для active = true
    // Подсказка: filter + map + fold (или sum)
    // ============================================================
    fn total_active_value(&self) -> f64 {
        // 🟢 Твой код здесь
        todo!("Задание 2: посчитай общую стоимость активных монет")
    }

    // ============================================================
    // 🎯 Задание 3: Создать HashMap <символ → стоимость>
    //
    // Все монеты (и активные, и нет)
    // Подсказка: map в кортежи + collect
    // ============================================================
    fn symbol_to_value(&self) -> HashMap<&str, f64> {
        // 🟢 Твой код здесь
        todo!("Задание 3: создай HashMap символ → стоимость")
    }

    // ============================================================
    // 🎯 Задание 4: Найти монету с максимальной стоимостью
    //
    // Стоимость = amount * price_usd. Вернуть символ.
    // Подсказка: max_by + map
    // ============================================================
    fn max_value_coin(&self) -> Option<&str> {
        // 🟢 Твой код здесь
        todo!("Задание 4: найди монету с макс. стоимостью")
    }

    // ============================================================
    // 🎯 Задание 5: Получить топ-3 монеты по стоимости
    //
    // Отсортировать по убыванию стоимости, взять 3
    // Подсказка: sort_by + take + collect
    // ============================================================
    fn top3_by_value(&self) -> Vec<&Coin> {
        // 🟢 Твой код здесь
        todo!("Задание 5: верни топ-3 монеты по стоимости")
    }
}

fn main() {
    let portfolio = Portfolio::new();

    println!("🎯 Анализ крипто-портфеля через итераторы\n");

    println!("1️⃣ Активные символы: {:?}", portfolio.active_symbols());
    println!("2️⃣ Общая стоимость: ${:.2}", portfolio.total_active_value());
    println!("3️⃣ Карта символ → стоимость: {:?}", portfolio.symbol_to_value());
    println!("4️⃣ Максимальная стоимость: {:?}", portfolio.max_value_coin());

    let top3 = portfolio.top3_by_value();
    println!("5️⃣ Топ-3:");
    for coin in &top3 {
        println!("   {} ({}): ${:.2}", coin.name, coin.symbol, coin.amount * coin.price_usd);
    }
}

// ============================================================
// ℹ️ Проверь себя — запусти cargo test после решения
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Portfolio {
        Portfolio::new()
    }

    #[test]
    fn test_active_symbols() {
        let p = setup();
        let symbols = p.active_symbols();
        assert_eq!(symbols, vec!["BTC", "ETH", "SOL", "DOT"]);
    }

    #[test]
    fn test_total_active_value() {
        let p = setup();
        let total = p.total_active_value();
        // 0.5*67000 + 5.0*3400 + 20.0*150 + 50.0*7.0 = 33500 + 17000 + 3000 + 350 = 53850
        assert!((total - 53850.0).abs() < 0.01);
    }

    #[test]
    fn test_symbol_to_value() {
        let p = setup();
        let map = p.symbol_to_value();
        assert!((map.get("BTC").unwrap() - 33500.0).abs() < 0.01);
        assert!((map.get("ADA").unwrap() - 45.0).abs() < 0.01);
        assert_eq!(map.len(), 5);
    }

    #[test]
    fn test_max_value_coin() {
        let p = setup();
        assert_eq!(p.max_value_coin(), Some("BTC"));
    }

    #[test]
    fn test_top3_by_value() {
        let p = setup();
        let top3 = p.top3_by_value();
        assert_eq!(top3.len(), 3);
        assert_eq!(top3[0].symbol, "BTC");
        assert_eq!(top3[1].symbol, "ETH");
        assert_eq!(top3[2].symbol, "SOL");
    }
}
