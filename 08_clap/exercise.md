# Упражнение: Свой CLI-калькулятор

## Задание

Создай CLI-программу `calc`, которая принимает:
- два числа (позиционные аргументы)
- флаг `--operation` или `-o` (add, sub, mul, div) — по умолчанию add
- флаг `--verbose` или `-v`

### Примеры запуска

```bash
cargo run -- 10 5
# Результат: 15

cargo run -- 10 5 -o mul
# Результат: 50

cargo run -- 10 5 -o div -v
# [INFO] Делим 10 на 5
# Результат: 2
```

### Подсказка

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "add")]
    operation: String,
    #[arg(short, long)]
    verbose: bool,
    a: f64,
    b: f64,
}
```

Удачи!
