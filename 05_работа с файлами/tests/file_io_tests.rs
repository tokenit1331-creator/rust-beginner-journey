// ============================================
// ✅ Тесты для урока 10: Работа с файлами
// ============================================

use std::fs;

// ===========================================
// ТЕСТ 1: Запись и чтение текста
// ===========================================
#[test]
fn test_write_and_read_text() {
    let path = "_test_write_read.txt";
    
    // Пишем
    fs::write(path, "Hello, Rust file I/O!").unwrap();
    
    // Читаем
    let content = fs::read_to_string(path).unwrap();
    assert_eq!(content, "Hello, Rust file I/O!");
    
    // Удаляем
    fs::remove_file(path).unwrap();
}

// ===========================================
// ТЕСТ 2: Пустой файл
// ===========================================
#[test]
fn test_empty_file() {
    let path = "_test_empty.txt";
    
    fs::write(path, "").unwrap();
    let content = fs::read_to_string(path).unwrap();
    assert_eq!(content, "");
    
    fs::remove_file(path).unwrap();
}

// ===========================================
// ТЕСТ 3: Файл не существует
// ===========================================
#[test]
fn test_file_not_found() {
    let result = fs::read_to_string("_nonexistent_file.txt");
    assert!(result.is_err());
}

// ===========================================
// ТЕСТ 4: Append к файлу
// ===========================================
#[test]
fn test_append_to_file() {
    use std::io::Write;
    
    let path = "_test_append.txt";
    
    // Пишем первую строку
    fs::write(path, "line1\n").unwrap();
    
    // Добавляем вторую
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(path)
        .unwrap();
    writeln!(file, "line2").unwrap();
    drop(file);
    
    // Проверяем
    let content = fs::read_to_string(path).unwrap();
    assert_eq!(content, "line1\nline2\n");
    
    fs::remove_file(path).unwrap();
}

// ===========================================
// ТЕСТ 5: Запись и чтение бинарных данных
// ===========================================
#[test]
fn test_binary_write_read() {
    let path = "_test_binary.bin";
    
    let data: [u8; 8] = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    fs::write(path, &data).unwrap();
    
    let loaded = fs::read(path).unwrap();
    assert_eq!(loaded.len(), 8);
    assert_eq!(loaded, data);
    
    fs::remove_file(path).unwrap();
}

// ===========================================
// ТЕСТ 6: Серде JSON (сериализация)
// ===========================================
#[test]
fn test_json_serialize() {
    use serde::{Serialize, Deserialize};
    
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestWallet {
        address: String,
        balance: f64,
    }
    
    let wallet = TestWallet {
        address: "0x123".to_string(),
        balance: 42.5,
    };
    
    let json = serde_json::to_string_pretty(&wallet).unwrap();
    let loaded: TestWallet = serde_json::from_str(&json).unwrap();
    
    assert_eq!(wallet, loaded);
}

// ===========================================
// ТЕСТ 7: JSON с Vec
// ===========================================
#[test]
fn test_json_with_vec() {
    use serde::{Serialize, Deserialize};
    
    #[derive(Debug, Serialize, Deserialize)]
    struct Portfolio {
        tokens: Vec<Token>,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    struct Token {
        symbol: String,
        amount: f64,
    }
    
    let portfolio = Portfolio {
        tokens: vec![
            Token { symbol: "ETH".into(), amount: 1.5 },
            Token { symbol: "BTC".into(), amount: 0.1 },
        ],
    };
    
    let json = serde_json::to_string_pretty(&portfolio).unwrap();
    let loaded: Portfolio = serde_json::from_str(&json).unwrap();
    
    assert_eq!(loaded.tokens.len(), 2);
    assert_eq!(loaded.tokens[0].symbol, "ETH");
}

// ===========================================
// ТЕСТ 8: Remove несуществующего файла
// ===========================================
#[test]
fn test_remove_nonexistent() {
    let result = fs::remove_file("_definitely_not_exists_12345.txt");
    assert!(result.is_err());
}

// ===========================================
// ТЕСТ 9: Metadata проверка
// ===========================================
#[test]
fn test_metadata_exists() {
    let path = "_test_meta.txt";
    
    // Файла нет
    assert!(fs::metadata(path).is_err());
    
    // Создаём
    fs::write(path, "test").unwrap();
    
    // Файл есть
    assert!(fs::metadata(path).is_ok());
    
    let meta = fs::metadata(path).unwrap();
    assert!(!meta.is_dir());
    assert!(meta.is_file());
    assert!(meta.len() > 0);
    
    fs::remove_file(path).unwrap();
}

// ===========================================
// ТЕСТ 10: CSV-like парсинг
// ===========================================
#[test]
fn test_csv_line_parsing() {
    let line = "2024-01-15,send,0.5,ETH,completed";
    let parts: Vec<&str> = line.split(',').collect();
    
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0], "2024-01-15");
    assert_eq!(parts[1], "send");
    
    let amount: f64 = parts[2].parse().unwrap();
    assert!((amount - 0.5).abs() < f64::EPSILON);
}

// ===========================================
// ТЕСТ 11: Перенос строки в конце
// ===========================================
#[test]
fn test_trailing_newline() {
    let path = "_test_newline.txt";
    
    fs::write(path, "line1\nline2\n").unwrap();
    let content = fs::read_to_string(path).unwrap();
    
    // .lines() не возвращает пустую строку для последнего \n
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 2);
    
    fs::remove_file(path).unwrap();
}

// ===========================================
// ТЕСТ 12: Большой текст
// ===========================================
#[test]
fn test_large_text() {
    let path = "_test_large.txt";
    
    // Создаём строку из 10 000 символов
    let large = "A".repeat(10_000);
    fs::write(path, &large).unwrap();
    
    let loaded = fs::read_to_string(path).unwrap();
    assert_eq!(loaded.len(), 10_000);
    
    fs::remove_file(path).unwrap();
}
