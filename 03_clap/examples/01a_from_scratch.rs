// ============================================================
// 🔵 САМЫЙ ПРОСТОЙ: парсим аргументы руками (БЕЗ clap)
// ============================================================
//
// Зачем это смотреть?
// Чтобы понять, КАКУЮ работу clap делает за тебя.
//
// ============================================================

fn main() {
    // ШАГ 1: Получаем аргументы
    let args: Vec<String> = std::env::args().collect();

    // ШАГ 2: Смотрим, что внутри
    println!("Все аргументы: {:?}", args);

    // ШАГ 3: Разбираем вручную
    let program = &args[0];
    println!("Имя программы: {}", program);

    if args.len() < 2 {
        println!("Ошибка: нужно передать имя!");
        println!("Использование: {} <имя>", program);
        return;
    }

    let name = &args[1];
    println!("Привет, {}!", name);

    // ШАГ 4: Пробуем распарсить флаг
    let mut count = 1;
    if args.len() >= 4 && args[2] == "--count" {
        count = args[3].parse().unwrap();
        for _ in 0..count {
            println!("  Привет, {}!", name);
        }
    } else {
        println!("Привет, {}!", name);
    }
}

// Запуск:
// cargo run --example 01a_from_scratch -- Артём
// cargo run --example 01a_from_scratch -- Артём --count 3
