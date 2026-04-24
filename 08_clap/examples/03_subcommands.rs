// ============================================================
// ПРИМЕР 3: СУБКОМАНДЫ (subcommands)
// ============================================================
// Как в git: git commit -m "..." и git push origin main
// Субкоманды — это вложенные структуры с #[derive(Subcommand)]
// ============================================================

use clap::{Parser, Subcommand};

/// CLI для управления задачами (как todo list)
#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Менеджер задач", version = "1.0")]
struct Cli {
    /// Субкоманда: add, list, done, delete
    #[command(subcommand)]
    command: Commands,
}

/// Все доступные субкоманды
#[derive(Subcommand)]
enum Commands {
    /// Добавить новую задачу
    Add {
        /// Текст задачи (позиционный аргумент)
        text: String,

        /// Приоритет: high, medium, low
        #[arg(short, long, default_value = "medium")]
        priority: String,
    },

    /// Показать список задач
    List {
        /// Фильтр по статусу
        #[arg(short, long)]
        status: Option<String>,
    },

    /// Отметить задачу выполненной
    Done {
        /// ID задачи
        id: u32,
    },

    /// Удалить задачу
    Delete {
        /// ID задачи
        id: u32,

        /// Не спрашивать подтверждение
        #[arg(short, long)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { text, priority } => {
            println!("✅ Добавлена задача: \"{}\" (приоритет: {})", text, priority);
        }
        Commands::List { status } => {
            match status {
                Some(s) => println!("📋 Список задач со статусом: {}", s),
                None => println!("📋 Список всех задач"),
            }
        }
        Commands::Done { id } => {
            println!("✔️ Задача #{} отмечена как выполненная", id);
        }
        Commands::Delete { id, force } => {
            if force {
                println!("🗑️ Задача #{} удалена без подтверждения", id);
            } else {
                println!("🗑️ Задача #{} удалена", id);
            }
        }
    }
}

// Примеры запуска:
// cargo run -- add "Купить хлеб" -p high
// cargo run -- list
// cargo run -- list -s done
// cargo run -- done 1
// cargo run -- delete 1 -f
