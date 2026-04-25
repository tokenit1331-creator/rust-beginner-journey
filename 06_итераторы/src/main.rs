fn main() {
    println!("=== Урок 11: Итераторы и методы итераторов ===\n");

    let examples = vec![
        ("01_basic_iterators",   "Базовые итераторы: iter, into_iter, iter_mut"),
        ("02_map_filter",        "map, filter, fold, take, skip"),
        ("03_chain_zip_flatmap", "chain, zip, flat_map, rev, step_by"),
        ("04_collect_and_own",   "collect, HashMap, ownership"),
        ("exercise",             "🎯 Задание — крипто-портфель"),
    ];

    for (i, (name, desc)) in examples.iter().enumerate() {
        println!("{}. {} — {}", i + 1, name, desc);
    }

    println!("\nЗапусти пример: cargo run --example <название>");
    println!("Например: cargo run --example 01_basic_iterators");
}
