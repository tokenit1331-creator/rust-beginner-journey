# Урок: clap — парсинг аргументов командной строки

## Что такое clap?

**clap** (Command Line Argument Parser) — это самая популярная библиотека в Rust 
для парсинга аргументов командной строки.

## Установка

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
```

## Два подхода

1. **Derive (макросы)** — простой и современный (рекомендуется)
2. **Builder Pattern** — гибкий, без макросов

## Файлы

- `src/main.rs` — пример с derive-макросами
- `examples/02_builder_pattern.rs` — пример с Builder Pattern
