// ============================================================
// ✅ Тесты для урока по итераторам
// ============================================================

#[derive(Debug, Clone, PartialEq)]
struct Coin {
    name: String,
    price: f64,
}

fn get_coins() -> Vec<Coin> {
    vec![
        Coin { name: "BTC".into(), price: 67000.0 },
        Coin { name: "ETH".into(), price: 3400.0  },
        Coin { name: "SOL".into(), price: 150.0   },
    ]
}

#[test]
fn test_iter_basic() {
    let coins = get_coins();
    let mut iter = coins.iter();
    assert_eq!(iter.next(), Some(&Coin { name: "BTC".into(), price: 67000.0 }));
    assert_eq!(iter.next(), Some(&Coin { name: "ETH".into(), price: 3400.0 }));
    assert_eq!(iter.next(), Some(&Coin { name: "SOL".into(), price: 150.0 }));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_map_double() {
    let prices = vec![1.0, 2.0, 3.0];
    let doubled: Vec<f64> = prices.iter().map(|x| x * 2.0).collect();
    assert_eq!(doubled, vec![2.0, 4.0, 6.0]);
}

#[test]
fn test_filter_active() {
    let coins = get_coins();
    let expensive: Vec<&Coin> = coins.iter().filter(|c| c.price > 1000.0).collect();
    assert_eq!(expensive.len(), 2);
    assert_eq!(expensive[0].name, "BTC");
    assert_eq!(expensive[1].name, "ETH");
}

#[test]
fn test_fold_sum() {
    let nums = vec![1, 2, 3, 4, 5];
    let sum: i32 = nums.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 15);
}

#[test]
fn test_take() {
    let coins = get_coins();
    let first_two: Vec<&Coin> = coins.iter().take(2).collect();
    assert_eq!(first_two.len(), 2);
}

#[test]
fn test_skip() {
    let coins = get_coins();
    let after_first: Vec<&Coin> = coins.iter().skip(1).collect();
    assert_eq!(after_first.len(), 2);
    assert_eq!(after_first[0].name, "ETH");
}

#[test]
fn test_chain() {
    let a = vec![1, 2];
    let b = vec![3, 4];
    let chained: Vec<i32> = a.iter().chain(b.iter()).map(|&x| x).collect();
    assert_eq!(chained, vec![1, 2, 3, 4]);
}

#[test]
fn test_zip() {
    let coins = vec!["BTC", "ETH"];
    let prices = vec![67000.0, 3400.0];
    let zipped: Vec<(&str, f64)> = coins.iter().zip(prices.iter()).map(|(&c, &p)| (c, p)).collect();
    assert_eq!(zipped, vec![("BTC", 67000.0), ("ETH", 3400.0)]);
}

#[test]
fn test_flat_map() {
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<i32> = nested.iter().flat_map(|x| x.iter()).map(|&x| x).collect();
    assert_eq!(flat, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_rev() {
    let nums = vec![1, 2, 3];
    let rev: Vec<i32> = nums.iter().rev().map(|&x| x).collect();
    assert_eq!(rev, vec![3, 2, 1]);
}

#[test]
fn test_collect_hashmap() {
    use std::collections::HashMap;
    let coins = get_coins();
    let map: HashMap<&str, f64> = coins.iter().map(|c| (c.name.as_str(), c.price)).collect();
    assert_eq!(map.get("BTC"), Some(&67000.0));
    assert_eq!(map.get("ETH"), Some(&3400.0));
    assert_eq!(map.len(), 3);
}

#[test]
fn test_into_iter_consumes() {
    let coins = get_coins();
    let owned: Vec<Coin> = coins.into_iter().collect();
    assert_eq!(owned.len(), 3);
    // coins больше не доступен — владение забрано
}
