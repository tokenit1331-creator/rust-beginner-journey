// ============================================================
// ПРИМЕР 7: ГЕНЕРАЦИЯ SHELL-ДОПОЛНЕНИЙ
// ============================================================
// clap умеет генерировать автодополнение для:
//   Bash, Zsh, Fish, PowerShell, Elvish
// ============================================================

use clap::{Parser, CommandFactory};
use clap_complete::{generate, Shell};
use std::io;

/// CLI с автодополнением для shell
#[derive(Parser)]
#[command(name = "mycli", version = "1.0")]
struct Cli {
    /// Субкоманда
    #[command(subcommand)]
    command: Option<Commands>,

    /// Сгенерировать автодополнение для shell
    /// Пример: mycli --completion bash > /etc/bash_completion.d/mycli
    #[arg(long = "completion", value_parser = ["bash", "zsh", "fish", "powershell", "elvish"])]
    completion: Option<String>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Создать ресурс
    Create {
        /// Имя ресурса
        name: String,
        /// Тип ресурса
        #[arg(short, long, default_value = "default")]
        r#type: String,
    },
    /// Удалить ресурс
    Delete {
        /// ID ресурса
        id: u32,
        /// Не спрашивать подтверждение
        #[arg(short, long)]
        force: bool,
    },
    /// Показать информацию
    Info {
        /// ID ресурса
        id: u32,
        /// Подробный вывод
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // Если запрошена генерация дополнений
    if let Some(shell) = cli.completion {
        let shell = match shell.as_str() {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            "fish" => Shell::Fish,
            "powershell" => Shell::PowerShell,
            "elvish" => Shell::Elvish,
            _ => unreachable!(),
        };
        generate(shell, &mut Cli::command(), "mycli", &mut io::stdout());
        return;
    }

    // Обычная работа
    match cli.command {
        Some(Commands::Create { name, r#type }) => {
            println!("✅ Создан ресурс '{}' типа '{}'", name, r#type);
        }
        Some(Commands::Delete { id, force }) => {
            if force {
                println!("🗑️ Ресурс #{} удалён (без подтверждения)", id);
            } else {
                println!("🗑️ Ресурс #{} удалён", id);
            }
        }
        Some(Commands::Info { id, verbose }) => {
            if verbose {
                println!("📄 Информация о ресурсе #{} (подробно)", id);
            } else {
                println!("📄 Информация о ресурсе #{}", id);
            }
        }
        None => {
            println!("mycli v1.0. Используйте --help для справки");
        }
    }
}

// Установка автодополнения:
// Bash:   mycli --completion bash > /etc/bash_completion.d/mycli
// Zsh:    mycli --completion zsh > /usr/share/zsh/site-functions/_mycli
// Fish:   mycli --completion fish > ~/.config/fish/completions/mycli.fish
// После этого: наберите mycli + TAB и увидите автодополнение!
