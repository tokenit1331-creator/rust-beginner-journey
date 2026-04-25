// ============================================
// 🎯 Задание: Крипто-кошелёк с файловым I/O
// ============================================
//
// КОНТЕКСТ: Ты пишешь простой крипто-кошелёк,
// который сохраняет состояние между запусками.
//
// ЗАДАНИЕ: Реализуй функции todo!() ниже.
//
// ПОДСКАЗКИ (не читай сразу — попробуй сам!):
// 💡 Используй fs::write() и fs::read_to_string()
// 💡 Используй serde_json для сериализации
// 💡 Используй ? для обработки ошибок
// 💡 Не забудь удалить временные файлы в конце

use serde::{Deserialize, Serialize};
use std::fs;

// ===========================================
// Структуры данных
// ===========================================

/// Кошелёк с балансом и историей
#[derive(Debug, Serialize, Deserialize)]
struct Wallet {
    /// Адрес кошелька
    address: String,
    /// Баланс в ETH
    balance: f64,
    /// История транзакций
    history: Vec<Transaction>,
}

/// Одна транзакция
#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    /// Тип: send или receive
    tx_type: String,
    /// Сумма в ETH
    amount: f64,
    /// Кому / от кого
    counterparty: String,
    /// Статус: pending или completed
    status: String,
}

// ===========================================
// Функции, которые нужно реализовать
// ===========================================

/// 1. Сохраняет кошелёк в файл в формате JSON
///
/// ПОЧЕМУ: Без сохранения все данные потеряются при выходе.
/// JSON выбран, потому что его читают и люди, и программы.
///
/// Путь: "wallet_state.json"
fn save_wallet(wallet: &Wallet) -> std::io::Result<()> {
    // 🎯 ТВОЙ КОД:
    // 1. Преобразуй wallet в JSON строку (serde_json::to_string_pretty)
    // 2. Запиши JSON в файл "wallet_state.json" (fs::write)
    // 3. Верни Ok(())
    todo!()  // <-- Удали эту строку и напиши свой код
}

/// 2. Загружает кошелёк из файла
///
/// Если файла нет — возвращает None.
/// Если файл есть, но не парсится — возвращает None.
///
/// ПОЧЕМУ: При первом запуске файла нет — это нормально.
/// Мы просто создадим новый кошелёк.
fn load_wallet() -> Option<Wallet> {
    // 🎯 ТВОЙ КОД:
    // 1. Попробуй прочитать файл "wallet_state.json"
    // 2. Если файла нет — верни None
    // 3. Если файл есть — распарсь JSON в Wallet
    // 4. Верни Some(wallet)
    todo!()  // <-- Удали эту строку и напиши свой код
}

/// 3. Добавляет транзакцию в историю
///
/// ПОЧЕМУ: Каждая транзакция меняет баланс.
/// Отправка — уменьшает, получение — увеличивает.
fn add_transaction(wallet: &mut Wallet, tx: Transaction) {
    // 🎯 ТВОЙ КОД:
    // 1. Обнови balance (+ для receive, - для send)
    // 2. Добавь tx в wallet.history
    // 3. Установи статус "completed"
    todo!()  // <-- Удали эту строку и напиши свой код
}

/// 4. Выводит отчёт по кошельку
///
/// ПОЧЕМУ: Пользователь хочет видеть, что происходит.
/// Красивый вывод — признак профессионального инструмента.
fn print_report(wallet: &Wallet) {
    println!("📋 ОТЧЁТ ПО КОШЕЛЬКУ");
    println!("   Адрес: {}", wallet.address);
    println!("   Баланс: {} ETH", wallet.balance);
    println!("   Транзакций: {}", wallet.history.len());

    // 🎯 ТВОЙ КОД (дополнительно):
    // Выведи последние 3 транзакции
    // Для каждой: тип, сумма, контрагент, статус
}

// ===========================================
// ТЕСТИРОВАНИЕ
// ===========================================
//
// Раскомментируй и запусти cargo test, чтобы проверить своё решение

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load() {
        let wallet = Wallet {
            address: "0xTest".to_string(),
            balance: 10.0,
            history: vec![],
        };

        // Сохраняем
        save_wallet(&wallet).expect("Не удалось сохранить");

        // Загружаем
        let loaded = load_wallet().expect("Не удалось загрузить");

        // Проверяем
        assert_eq!(loaded.address, "0xTest");
        assert_eq!(loaded.balance, 10.0);

        // Удаляем мусор
        fs::remove_file("wallet_state.json").ok();
    }

    #[test]
    fn test_add_transaction() {
        let mut wallet = Wallet {
            address: "0xTest".to_string(),
            balance: 10.0,
            history: vec![],
        };

        let tx = Transaction {
            tx_type: "send".to_string(),
            amount: 2.0,
            counterparty: "0xFriend".to_string(),
            status: "pending".to_string(),
        };

        add_transaction(&mut wallet, tx);

        assert_eq!(wallet.balance, 8.0);  // 10 - 2 = 8
        assert_eq!(wallet.history.len(), 1);
        assert_eq!(wallet.history[0].status, "completed");
    }
}

// ===========================================
// ЗАПУСК
// ===========================================
//
// Когда всё реализуешь, запусти:
//   cargo run --example exercise
//
// А затем тесты:
//   cargo test

fn main() -> std::io::Result<()> {
    println!("🎯 Задание: Крипто-кошелёк с файловым I/O");
    println!();

    // Пробуем загрузить существующий кошелёк
    let mut wallet = load_wallet().unwrap_or(Wallet {
        address: "0x742d35Cc6634C0532925a3b844Bc9e7595f3bD1f".to_string(),
        balance: 100.0,
        history: vec![],
    });

    println!("✅ Кошелёк загружен: {} ({} ETH)", wallet.address, wallet.balance);

    // Добавляем тестовые транзакции
    add_transaction(&mut wallet, Transaction {
        tx_type: "send".to_string(),
        amount: 10.0,
        counterparty: "0xFriend".to_string(),
        status: "pending".to_string(),
    });

    add_transaction(&mut wallet, Transaction {
        tx_type: "receive".to_string(),
        amount: 25.0,
        counterparty: "0xExchange".to_string(),
        status: "pending".to_string(),
    });

    add_transaction(&mut wallet, Transaction {
        tx_type: "send".to_string(),
        amount: 5.0,
        counterparty: "0xMerchant".to_string(),
        status: "pending".to_string(),
    });

    // Выводим отчёт
    print_report(&wallet);

    // Сохраняем
    save_wallet(&wallet)?;
    println!("✅ Кошелёк сохранён в wallet_state.json");

    // Убираем за собой
    fs::remove_file("wallet_state.json")?;
    println!("🗑️ Временный файл удалён");

    println!();
    println!("🎉 Задание выполнено! Запусти cargo test для проверки.");

    Ok(())
}
