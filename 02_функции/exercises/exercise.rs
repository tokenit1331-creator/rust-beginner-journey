// ============================================================
// 🎯 УПРАЖНЕНИЯ: ФУНКЦИИ
// ============================================================
//
// ДЛЯ НОВИЧКА: Здесь 5 заданий — от простого к сложному.
// Каждое задание — это функция, которую нужно дописать.
// В начале каждой задачи написано что нужно сделать.
// ============================================================

fn test_exercise() {
    println!("=== Проверка заданий ===\n");
    assert_eq!(calculate_area(5.0, 3.0), 15.0);
    println!("✅ Задание 1: площадь прямоугольника");
    assert_eq!(validate_password("Rust123!@#"), "✅ Пароль надёжный".to_string());
    assert_eq!(validate_password("short"), "❌ Пароль слишком короткий".to_string());
    println!("✅ Задание 2: проверка пароля");
    let nums = vec![3, 7, 1, 9, 4];
    assert_eq!(find_largest(&nums), &9);
    println!("✅ Задание 3: максимальное число");
    let result = process_with_callback(5, |x| x * x);
    assert_eq!(result, 25);
    println!("✅ Задание 4: колбэк");
    let sum = process_transactions_fold(&[10.0, 20.0, 30.0], |acc, val| acc + val, 0.0);
    assert_eq!(sum, 60.0);
    println!("✅ Задание 5: fold");
    println!("\n🎉 Все задания решены!");
}

fn calculate_area(width: f64, height: f64) -> f64 {
    todo!()
}

fn validate_password(password: &str) -> String {
    todo!()
}

fn find_largest(numbers: &[i32]) -> Option<&i32> {
    todo!()
}

fn process_with_callback(x: i32, f: impl Fn(i32) -> i32) -> i32 {
    todo!()
}

fn process_transactions_fold(items: &[f64], f: impl Fn(f64, f64) -> f64, initial: f64) -> f64 {
    todo!()
}

fn main() {
    println!("🎯 Решаем задания по функциям!");
    println!("Отредактируй функции выше, а потом запусти:");
    println!("  cargo run --example exercise\n");
    test_exercise();
}
