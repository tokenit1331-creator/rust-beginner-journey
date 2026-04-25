// ─── Упражнения: Функции и управление в Rust ───

// ЗАДАНИЕ 1: Функция приветствия
fn greet(name: String, lang: &str) -> String {
    todo!()
}

// ЗАДАНИЕ 2: Валидация пароля
fn validate_password(password: &str) -> Result<(), String> {
    todo!()
}

// ЗАДАНИЕ 3: Обобщённый max
fn my_max<T: PartialOrd>(a: T, b: T) -> T {
    todo!()
}

// ЗАДАНИЕ 4: Замыкание-фильтр
fn filter_list<F>(items: &[i32], predicate: F) -> Vec<i32>
where F: Fn(&i32) -> bool {
    todo!()
}

// ЗАДАНИЕ 5: Композиция
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where F: Fn(A) -> B, G: Fn(B) -> C {
    todo!()
}

fn main() {
    println!("🎯 Упражнения по функциям");
    println!("Открой файл exercises/exercise.rs и напиши решения!");
}