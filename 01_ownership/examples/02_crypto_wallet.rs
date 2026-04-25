// ┌─────────────────────────────────────────────────────────────────────┐
// │    02_crypto_wallet.rs — Владение в крипто-кошельке                │
// │                                                                     │
// │ Запуск: cargo run --example 02_crypto_wallet                        │
// │                                                                     │
// │ Реальный пример: кошелёк с токенами, переводы, баланс              │
// └─────────────────────────────────────────────────────────────────────┘

use std::collections::HashMap;

// =====================================================================
// 🪙 СТРУКТУРЫ
// =====================================================================

/// Кошелёк пользователя
#[derive(Debug, Clone)]
struct Wallet {
    address: String,
    owner: String,
    tokens: HashMap<String, f64>, // тикер -> количество
}

/// Транзакция перевода
#[derive(Debug)]
struct Transfer {
    from: String,
    to: String,
    token: String,
    amount: f64,
    signature: String, // хеш транзакции
}

// =====================================================================
// 🔵 БАЗОВЫЕ ОПЕРАЦИИ (без заимствования — move везде)
// =====================================================================

/// ❌ НЕБЕЗОПАСНО: забирает владение кошельком
fn _bad_check_balance(wallet: Wallet) {
    println!("💰 Баланс кошелька {}:", wallet.address);
    for (token, balance) in &wallet.tokens {
        println!("   {}: {}", token, balance);
    }
    // ⚠️ wallet УМИРАЕТ здесь!
    // После вызова этой функции вызывающий теряет кошелёк!
}

/// ✅ БЕЗОПАСНО: читает баланс по ссылке
fn check_balance(wallet: &Wallet) {
    println!("💰 Баланс кошелька {}:", wallet.address);
    for (token, balance) in &wallet.tokens {
        println!("   {}: {}", token, balance);
    }
    // wallet — это ссылка, ничего не удаляется
}

// =====================================================================
// 🟡 ОПЕРАЦИИ С ИЗМЕНЕНИЕМ (&mut T)
// =====================================================================

/// ✅ БЕЗОПАСНО: пополняет баланс через изменяемую ссылку
fn deposit(wallet: &mut Wallet, token: &str, amount: f64) {
    let balance = wallet.tokens.entry(token.to_string()).or_insert(0.0);
    *balance += amount;
    println!("📥 Пополнение: +{} {} на адрес {}", amount, token, wallet.address);
}

/// ✅ БЕЗОПАСНО: списывает токены, возвращает результат
fn withdraw(wallet: &mut Wallet, token: &str, amount: f64) -> Result<(), String> {
    match wallet.tokens.get_mut(token) {
        Some(balance) if *balance >= amount => {
            *balance -= amount;
            println!("📤 Снятие: -{} {}", amount, token);
            Ok(())
        }
        Some(balance) => Err(format!(
            "❌ Недостаточно {}. Доступно: {}, запрошено: {}",
            token, balance, amount
        )),
        None => Err(format!("❌ Токен {} не найден в кошельке", token)),
    }
}

// =====================================================================
// 🟠 ПЕРЕВОД МЕЖДУ КОШЕЛЬКАМИ (сложный пример)
// =====================================================================

/// Переводит токены между двумя кошельками
/// Принимает ДВЕ изменяемые ссылки!
fn transfer_between(
    from: &mut Wallet,
    to: &mut Wallet,
    token: &str,
    amount: f64,
) -> Result<Transfer, String> {
    // 1. Проверяем и списываем
    withdraw(from, token, amount)?;

    // 2. Зачисляем
    deposit(to, token, amount);

    // 3. Создаём запись о транзакции
    let transfer = Transfer {
        from: from.address.clone(),
        to: to.address.clone(),
        token: token.to_string(),
        amount,
        signature: format!("0x{:x}", rand::random::<u64>()), // упрощённо
    };

    println!("\n✅ Перевод выполнен!");
    println!("   {} {} -> {}", amount, token, transfer.to);
    println!("   Сигнатура: {}", transfer.signature);

    Ok(transfer)
}

// =====================================================================
// 🔴 ДЕМОНСТРАЦИЯ ОШИБКИ: две &mut ссылки
// =====================================================================

/// ❌ НЕ СКОМПИЛИРУЕТСЯ: демонстрирует защиту от гонок
fn _bad_transfer_mut() {
    let mut alice = Wallet {
        address: String::from("0xAlice"),
        owner: String::from("Alice"),
        tokens: HashMap::from([(String::from("USDT"), 100.0)]),
    };

    // ⚠️ Пытаемся взять две &mut ссылки на ОДИН кошелёк
    let r1 = &mut alice;
    let r2 = &mut alice; // ❌ ОШИБКА: cannot borrow `alice` as mutable more than once

    r1.tokens.insert(String::from("BTC"), 1.0);
    r2.tokens.insert(String::from("ETH"), 10.0); // ❌ ОШИБКА
}

// =====================================================================
// 🟢 БЕЗОПАСНЫЙ ВАРИАНТ: по очереди, а не одновременно
// =====================================================================

fn safe_mut_usage() {
    let mut wallet = Wallet {
        address: String::from("0xSafe"),
        owner: String::from("Safe User"),
        tokens: HashMap::new(),
    };

    // Первая &mut ссылка
    {
        let r1 = &mut wallet;
        deposit(r1, "USDT", 500.0);
    } // r1 умирает здесь

    // Вторая &mut ссылка — теперь можно!
    {
        let r2 = &mut wallet;
        deposit(r2, "BTC", 2.5);
    } // r2 умирает здесь

    // Теперь можно читать
    check_balance(&wallet);
}

// =====================================================================
// 🆚 СРАВНЕНИЕ: move vs clone vs reference
// =====================================================================

fn compare_approaches() {
    println!("\n=== 🆚 СРАВНЕНИЕ ПОДХОДОВ ===");

    let wallet = Wallet {
        address: String::from("0xCompare"),
        owner: String::from("Test"),
        tokens: HashMap::from([
            (String::from("USDT"), 1000.0),
            (String::from("ETH"), 5.0),
        ]),
    };

    // ❌ ПЛОХО: clone на пустом месте
    let _cloned = wallet.clone(); // глубокая копия всей структуры!
    println!("❌ clone() создал полную копию — {} потрачено впустую", std::mem::size_of::<Wallet>());

    // ✅ ХОРОШО: ссылка
    check_balance(&wallet); // ничего не копируется
    println!("✅ Ссылка & — просто указатель, 8 байт на стеке");

    // ✅ ХОРОШО: можно много раз
    check_balance(&wallet);
    check_balance(&wallet);
    check_balance(&wallet);
    println!("✅ Одну ссылку можно использовать сколько угодно");
}

// =====================================================================
// 🧪 ГЛАВНАЯ
// =====================================================================

fn main() {
    println!("═══════════════════════════════════════════════════════");
    println!("  🪙 КРИПТО-КОШЕЛЁК: ВЛАДЕНИЕ НА ПРАКТИКЕ");
    println!("═══════════════════════════════════════════════════════");

    // Создаём кошельки
    let mut alice = Wallet {
        address: String::from("0xAlice"),
        owner: String::from("Alice"),
        tokens: HashMap::from([
            (String::from("USDT"), 1000.0),
            (String::from("BTC"), 0.5),
        ]),
    };

    let mut bob = Wallet {
        address: String::from("0xBob"),
        owner: String::from("Bob"),
        tokens: HashMap::from([
            (String::from("USDT"), 500.0),
            (String::from("ETH"), 2.0),
        ]),
    };

    // 1️⃣ Читаем балансы (ссылками)
    println!("\n📖 ЧТЕНИЕ БАЛАНСОВ:");
    check_balance(&alice);
    check_balance(&bob);

    // 2️⃣ Переводим USDT от Alice к Bob
    println!("\n💸 ПЕРЕВОД:");
    match transfer_between(&mut alice, &mut bob, "USDT", 200.0) {
        Ok(tx) => println!("✅ Транзакция: {}", tx.signature),
        Err(e) => println!("{}", e),
    }

    // 3️⃣ Проверяем балансы после перевода
    println!("\n📖 БАЛАНСЫ ПОСЛЕ ПЕРЕВОДА:");
    check_balance(&alice);
    check_balance(&bob);

    // 4️⃣ Пробуем снять больше, чем есть (ошибка!)
    println!("\n⚠️ ПОПЫТКА СНЯТИЯ БОЛЬШЕ БАЛАНСА:");
    match withdraw(&mut alice, "BTC", 999.0) {
        Ok(()) => println!("✅ Снято!"),
        Err(e) => println!("{}", e),
    }

    // 5️⃣ Сравнение подходов
    compare_approaches();

    // 6️⃣ Безопасное использование
    println!("\n🔒 БЕЗОПАСНОЕ ИСПОЛЬЗОВАНИЕ:");
    safe_mut_usage();

    println!("\n═══════════════════════════════════════════════════════");
    println!("  ✅ Все операции выполнены!");
    println!("═══════════════════════════════════════════════════════");
}
