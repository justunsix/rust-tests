# AI-102 Quiz TUI (Rust)

Small terminal quiz app that reads multiple-choice questions from
`AI-102-Quiz.org`, lets you answer with number keys, and prints a final score
report.

Credit for quiz goes to Alex Ivanov, author of
[Developing Solutions for Microsoft Azure AZ-204 Exam Guide: Discover the essentials for success when developing and maintaining cloud-based solutions on Azure](https://www.goodreads.com/book/show/63335084-developing-solutions-for-microsoft-azure-az-204-exam-guide).
Quiz based on material as of 2025-11, though the learn material has been updated
due to changes in Microsoft Azure Foundry.

## Features

- Runs the full quiz in one terminal session
- Answer using number keys (`1`-`5`)
- Uses answer key from the same org file
- Writes local CSV files for answer key, per-question results, and session
  summaries

## Project Structure

- `src/main.rs`: quiz parser, terminal interaction, scoring, CSV persistence

Data to be generated for tracking answers and user sessions:

- `data/answer_key.csv`: generated answer key from parsed quiz
- `data/quiz_results.csv`: appended per-question results per session
- `data/session_summary.csv`: appended score summary per session

## Run with Cargo

```bash
cargo run
```

Optional arguments:

```bash
cargo run -- ./AI-102-Quiz.org ./data
```

- Arg 1: path to quiz org file with questions and answers
- Arg 2: output data directory

## Run with `mise` tasks

Use the local `mise.toml` tasks in this folder.

```bash
mise run build
mise run dev
```

`dev` runs the quiz with defaults:
