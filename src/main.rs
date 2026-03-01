use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command; // Модуль для запуска внешних команд
use std::sync::{Arc, Mutex};
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
#[command(name = "rust-finder")]
#[command(about = "Поиск и очистка Rust проектов через cargo clean", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Поиск Rust проектов и сохранение их путей
    Find {
        /// Где начинать поиск
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Дополнительные папки для игнорирования (флаг -e)
        #[arg(short, long)]
        exclude: Vec<String>,
    },
    /// Очистка найденных проектов через 'cargo clean'
    Clear,
}

const CACHE_FILE: &str = ".found_projects.txt";

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Find { path, exclude } => {
            println!("🔍 Поиск Rust проектов в: {}", path.display());
            let start = Instant::now();

            let found = find_cargo_projects(&path, exclude);
            let duration = start.elapsed();

            if found.is_empty() {
                println!("❌ Проектов не найдено.");
            } else {
                println!("✅ Найдено проектов: {}", found.len());
                for p in &found {
                    println!("  - {}", p.display());
                }

                save_to_cache(&found);
                println!("\n⏱️ Поиск занял: {:?}", duration);
                println!("💡 Теперь запустите: ./cargo-purge clear");
            }
        }
        Commands::Clear => {
            let paths = load_from_cache();
            if paths.is_empty() {
                println!("⚠️ Список пуст. Сначала запустите 'find'.");
                return;
            }

            println!("🧹 Начинаю 'cargo clean' для {} проектов...", paths.len());

            for p in paths {
                println!("🚀 Очистка в: {}", p.display());

                // Запускаем cargo clean внутри директории проекта
                let status = Command::new("cargo")
                    .arg("clean")
                    .current_dir(&p) // <--- Это магия: команда выполнится ТАМ
                    .status();

                match status {
                    Ok(s) if s.success() => println!("  ✅ Успешно"),
                    Ok(s) => eprintln!("  ⚠️ Завершилось с ошибкой: {}", s),
                    Err(e) => eprintln!("  ❌ Не удалось запустить cargo: {}", e),
                }
            }

            let _ = fs::remove_file(CACHE_FILE);
            println!("✨ Все операции завершены.");
        }
    }
}

fn find_cargo_projects(root: &Path, exclude_list: Vec<String>) -> Vec<PathBuf> {
    let custom_excludes: HashSet<String> = exclude_list.into_iter().collect();
    let projects = Arc::new(Mutex::new(Vec::new()));
    let projects_clone = Arc::clone(&projects);

    let walker = WalkDir::new(root).into_iter().filter_entry(move |e| {
        if should_skip(e, &custom_excludes) {
            return false;
        }

        if e.path().join("Cargo.toml").is_file() {
            projects_clone.lock().unwrap().push(e.path().to_path_buf());
            return false; // Нашли корень проекта, глубже не идем
        }

        true
    });

    for _ in walker.filter_map(|e| e.ok()) {}

    let mut result = projects.lock().unwrap().clone();
    result.sort();
    result
}

fn should_skip(entry: &DirEntry, custom_excludes: &HashSet<String>) -> bool {
    const DEFAULT_IGNORES: &[&str] = &[
        ".git",
        "target",
        "node_modules",
        ".idea",
        ".vscode",
        "build",
        "venv",
    ];
    let file_name = entry.file_name().to_string_lossy();
    DEFAULT_IGNORES.contains(&file_name.as_ref()) || custom_excludes.contains(file_name.as_ref())
}

fn save_to_cache(paths: &[PathBuf]) {
    let data: Vec<String> = paths
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    let _ = fs::write(CACHE_FILE, data.join("\n"));
}

fn load_from_cache() -> Vec<PathBuf> {
    fs::read_to_string(CACHE_FILE)
        .unwrap_or_default()
        .lines()
        .map(PathBuf::from)
        .collect()
}
