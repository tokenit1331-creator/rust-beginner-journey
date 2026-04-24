# 💡 Советы от опытного разработчика: работа с файлами

## 1. Начинай с `fs::read_to_string()` и `fs::write()`

Это самое простое. Не усложняй, пока не упрёшься.

```rust
// 👍 Для 80% задач этого достаточно
let data = fs::read_to_string("config.toml")?;
fs::write("output.txt", &data)?;
```

## 2. Используй `.expect()` для прототипов, `?` для продакшена

```rust
// Прототип — быстро и понятно
let data = fs::read_to_string("keys.txt")
    .expect("Нет файла keys.txt! Создай его.");

// Продакшен — ошибка всплывёт наверх
fn load_keys() -> Result<String, io::Error> {
    let data = fs::read_to_string("keys.txt")?;
    Ok(data)
}
```

## 3. Для структур → JSON + Serde

Не пиши свой парсер. Serde — стандарт де-факто.

```rust
#[derive(Serialize, Deserialize)]
struct Wallet {
    address: String,
    balance: f64,
}

// Сохранить
let json = serde_json::to_string_pretty(&wallet)?;
fs::write("wallet.json", &json)?;

// Прочитать
let text = fs::read_to_string("wallet.json")?;
let wallet: Wallet = serde_json::from_str(&text)?;
```

## 4. Для ключей и хешей — бинарные файлы

Не храни приватные ключи как текст. Используй байты.

```rust
let key: [u8; 32] = /* приватный ключ */;
fs::write("key.bin", &key)?;  // 32 байта, а не 64 символа hex
```

## 5. Для логов — `append`

Не перезаписывай логи. Добавляй в конец.

```rust
let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open("transactions.log")?;
writeln!(file, "{}: {} {} токенов", time, action, amount)?;
```

## 6. Для больших файлов — `BufReader`

Если файл > 10 МБ — не читай целиком. Читай построчно.

```rust
let file = File::open("big_data.csv")?;
let reader = BufReader::new(file);
for line in reader.lines() {
    let line = line?;
    // обрабатываем строку
}
```

## 7. Не захардкоживай пути к файлам

Пусть пользователь указывает путь через аргумент CLI.

```rust
#[arg(short, long, default_value = "wallet.json")]
config: String,
```

## 8. Всегда проверяй, что файл существует

```rust
if fs::metadata("config.json").is_ok() {
    // файл есть — читаем
    let data = fs::read_to_string("config.json")?;
} else {
    // файла нет — создаём с дефолтом
    fs::write("config.json", DEFAULT_CONFIG)?;
}
```

## 9. Не храни секреты в коде

Никогда не пиши API-ключи и приватные ключи в исходниках.
Читай их из файла или переменных окружения.

## 10. Для конкурентного доступа — блокировки

Если несколько потоков пишут в один файл — используй `Mutex<File>` или специализированные решения.

## 11. Тестируй с временными файлами

```rust
#[test]
fn test_write_read() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.txt");
    fs::write(&path, "hello").unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "hello");
}
```

## 12. `writeln!` удобнее, чем `write!`

```rust
// Так — компилируется, но нет переноса
write!(file, "строка")?;

// Так — с переносом строки
writeln!(file, "строка")?;
```

## 13. Ошибка "No such file or directory" vs "Permission denied"

```rust
match fs::read_to_string("secret.txt") {
    Ok(data) => println!("Прочитано: {}", data),
    Err(e) if e.kind() == io::ErrorKind::NotFound => {
        println!("Файла нет — создаём...");
    }
    Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
        println!("Нет прав на чтение!");
    }
    Err(e) => println!("Ошибка: {}", e),
}
```

## 14. UTF-16 файлы

По умолчанию Rust читает UTF-8. Если файл в UTF-16 — используй `decode_utf16`.

## 15. Продакшен-чит: unwrap_or_default

```rust
let data = fs::read_to_string("optional.txt")
    .unwrap_or_default();  // если файла нет — пустая строка
```

## 16. Читай CSV через csv крейт, не руками

Для серьёзной работы с CSV используй `csv` крейт (с поддержкой экранирования).

## 17. Всегда закрывай файлы

Rust закрывает файл автоматически при выходе из области видимости.
Не вызывай `.close()` — его нет. Просто выйди из функции.

## 18. Кодировка имеет значение

```rust
// Прочитать как UTF-8 (если файл в cp1251 — будет кракозябра)
let text = fs::read_to_string("data.txt")?;

// Если кодировка другая — читай как байты и конвертируй
let bytes = fs::read("data.txt")?;
let text = String::from_utf8_lossy(&bytes);
```
