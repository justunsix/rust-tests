use crossterm::cursor;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{self, ClearType};
use csv::WriterBuilder;
use std::collections::HashMap;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Question {
    number: usize,
    module: String,
    text: String,
    options: Vec<String>,
    correct_index: Option<usize>,
}

#[derive(Debug, Clone)]
struct Score {
    correct_count: usize,
    answered_count: usize,
    total_count: usize,
}

#[derive(Debug, Clone)]
struct QuizRunResult {
    answers: Vec<Option<usize>>,
    completed: bool,
}

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> Result<Self, String> {
        terminal::enable_raw_mode().map_err(|e| format!("Failed to enable raw mode: {e}"))?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let quiz_path = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("./AI-102-Quiz.org"));
    let data_dir = args
        .get(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("./data"));

    let mut questions = parse_org_quiz(&quiz_path)?;
    if questions.is_empty() {
        return Err("No quiz questions were parsed from the org file".to_string());
    }

    let answer_map = parse_answer_key(&quiz_path)?;
    assign_answer_indices(&mut questions, &answer_map);

    println!(
        "Loaded {} questions from {}",
        questions.len(),
        quiz_path.display()
    );
    println!("Press Enter to start...");
    let mut start = String::new();
    io::stdin()
        .read_line(&mut start)
        .map_err(|e| format!("Failed to read input: {e}"))?;

    let result = run_quiz_terminal(&questions)?;
    let score = calculate_score(&questions, &result.answers);

    if score.answered_count > 0 {
        persist_csv_files(&data_dir, &questions, &result.answers, &score)?;
    }

    if !result.completed {
        println!("\nQuiz ended early. Showing results for answered questions only.");
    }
    print_report(&questions, &result.answers, &score);

    Ok(())
}

fn parse_org_quiz(path: &Path) -> Result<Vec<Question>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read org file '{}': {e}", path.display()))?;
    let lines: Vec<&str> = content.lines().collect();

    let mut questions = Vec::new();
    let mut current_module = String::new();
    let mut in_answers_section = false;
    let mut idx = 0usize;
    let mut question_number = 1usize;

    while idx < lines.len() {
        let line = lines[idx].trim();

        if line.starts_with("* AI-102 answers from Alex") {
            in_answers_section = true;
        }
        if in_answers_section {
            idx += 1;
            continue;
        }

        if line.starts_with("** ") {
            current_module = line.trim_start_matches("** ").trim().to_string();
            idx += 1;
            continue;
        }

        if is_question_line(line) {
            let question_text = line.to_string();
            idx += 1;

            let mut options = Vec::new();
            while idx < lines.len() {
                let next = lines[idx].trim();
                if next.starts_with("- ") {
                    options.push(next.trim_start_matches("- ").trim().to_string());
                    idx += 1;
                    continue;
                }
                if next.is_empty() {
                    idx += 1;
                    continue;
                }
                break;
            }

            if !options.is_empty() {
                questions.push(Question {
                    number: question_number,
                    module: current_module.clone(),
                    text: question_text,
                    options,
                    correct_index: None,
                });
                question_number += 1;
            }
            continue;
        }

        idx += 1;
    }

    Ok(questions)
}

fn parse_answer_key(path: &Path) -> Result<HashMap<usize, String>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read org file '{}': {e}", path.display()))?;
    let lines: Vec<&str> = content.lines().collect();

    let mut answer_map: HashMap<usize, String> = HashMap::new();
    let mut in_answers_section = false;
    let mut idx = 0usize;

    while idx < lines.len() {
        let line = lines[idx].trim();

        if line.starts_with("* AI-102 answers from Alex") {
            in_answers_section = true;
            idx += 1;
            continue;
        }

        if !in_answers_section {
            idx += 1;
            continue;
        }

        if let Some(number) = parse_answer_question_number(line)
            && let Some(answer_text) = find_next_answer_line(&lines, idx + 1)
        {
            answer_map.insert(number, answer_text);
        }

        idx += 1;
    }

    Ok(answer_map)
}

fn parse_answer_question_number(line: &str) -> Option<usize> {
    let dot_index = line.find('.')?;
    let number_part = line[..dot_index].trim();
    number_part.parse::<usize>().ok()
}

fn find_next_answer_line(lines: &[&str], start_idx: usize) -> Option<String> {
    let mut idx = start_idx;
    while idx < lines.len() {
        let line = lines[idx].trim();
        if line.is_empty() {
            idx += 1;
            continue;
        }
        if line.starts_with("A:") {
            return Some(extract_answer_text(line));
        }
        if !line.starts_with('*') && parse_answer_question_number(line).is_none() {
            return Some(extract_answer_text(line));
        }
        if parse_answer_question_number(line).is_some() || line.starts_with('*') {
            return None;
        }
        idx += 1;
    }
    None
}

fn extract_answer_text(answer_line: &str) -> String {
    let mut text = answer_line.trim().to_string();
    if text.starts_with("A:") {
        text = text.trim_start_matches("A:").trim().to_string();
    }
    text = text.replace("â€œ", "");
    text = text.replace("â€", "");
    text = text.replace('“', "");
    text = text.replace('”', "");

    if let Some(and_pos) = text.find(" and A:") {
        text = text[..and_pos].trim().to_string();
    }
    if let Some(extra_pos) = text.find(" - ") {
        text = text[..extra_pos].trim().to_string();
    }

    text
}

fn assign_answer_indices(questions: &mut [Question], answer_map: &HashMap<usize, String>) {
    for question in questions {
        if let Some(answer_text) = answer_map.get(&question.number) {
            question.correct_index = find_matching_option_index(&question.options, answer_text);
        }
    }
}

fn find_matching_option_index(options: &[String], answer_text: &str) -> Option<usize> {
    let normalized_answer = normalize(answer_text);

    for (idx, option) in options.iter().enumerate() {
        let normalized_option = normalize(option);
        if normalized_option.is_empty() {
            continue;
        }

        if normalized_answer.contains(&normalized_option)
            || normalized_option.contains(&normalized_answer)
        {
            return Some(idx);
        }
    }

    None
}

fn normalize(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut last_was_space = true;

    for ch in input.chars() {
        if ch.is_alphanumeric() {
            for lower in ch.to_lowercase() {
                output.push(lower);
            }
            last_was_space = false;
        } else if !last_was_space {
            output.push(' ');
            last_was_space = true;
        }
    }

    output.trim().to_string()
}

fn is_question_line(line: &str) -> bool {
    !line.is_empty() && line.ends_with('?') && !line.starts_with('*') && !line.starts_with('-')
}

fn run_quiz_terminal(questions: &[Question]) -> Result<QuizRunResult, String> {
    let _raw_guard = RawModeGuard::new()?;
    let mut stdout = io::stdout();
    let mut answers = vec![None; questions.len()];

    for (idx, question) in questions.iter().enumerate() {
        loop {
            draw_question(&mut stdout, idx + 1, questions.len(), question)?;
            stdout
                .flush()
                .map_err(|e| format!("Failed to flush stdout: {e}"))?;

            if let Event::Key(key_event) =
                event::read().map_err(|e| format!("Failed to read key event: {e}"))?
            {
                if key_event.kind != KeyEventKind::Press {
                    continue;
                }

                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        execute!(
                            stdout,
                            terminal::Clear(ClearType::All),
                            cursor::MoveTo(0, 0)
                        )
                        .map_err(|e| format!("Failed to clear terminal: {e}"))?;

                        return Ok(QuizRunResult {
                            answers,
                            completed: false,
                        });
                    }
                    KeyCode::Char(ch) if ch.is_ascii_digit() => {
                        let selected = ch.to_digit(10).unwrap_or(0) as usize;
                        if selected >= 1 && selected <= question.options.len() {
                            answers[idx] = Some(selected - 1);
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .map_err(|e| format!("Failed to clear terminal: {e}"))?;

    Ok(QuizRunResult {
        answers,
        completed: true,
    })
}

fn draw_question(
    stdout: &mut io::Stdout,
    current: usize,
    total: usize,
    question: &Question,
) -> Result<(), String> {
    let terminal_width = terminal::size()
        .map(|(width, _)| width as usize)
        .unwrap_or(100)
        .max(20);

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .map_err(|e| format!("Failed to draw question: {e}"))?;

    write!(stdout, "AI-102 Quiz ({}/{})\r\n", current, total)
        .map_err(|e| format!("Failed to write header: {e}"))?;
    write!(stdout, "Module: {}\r\n", question.module)
        .map_err(|e| format!("Failed to write module: {e}"))?;
    write!(stdout, "\r\n").map_err(|e| format!("Failed to write spacing: {e}"))?;
    for line in wrap_with_prefix(
        &format!("Q{}: ", question.number),
        &question.text,
        terminal_width,
    ) {
        write!(stdout, "{}\r\n", line).map_err(|e| format!("Failed to write question: {e}"))?;
    }
    write!(stdout, "\r\n").map_err(|e| format!("Failed to write spacing: {e}"))?;

    for (index, option) in question.options.iter().enumerate() {
        let prefix = format!("  {}. ", index + 1);
        for line in wrap_with_prefix(&prefix, option, terminal_width) {
            write!(stdout, "{}\r\n", line).map_err(|e| format!("Failed to write option: {e}"))?;
        }
    }

    write!(stdout, "\r\n").map_err(|e| format!("Failed to write spacing: {e}"))?;
    for line in wrap_with_prefix(
        "",
        "Press number key (1-5). Press q to quit.",
        terminal_width,
    ) {
        write!(stdout, "{}\r\n", line).map_err(|e| format!("Failed to write prompt: {e}"))?;
    }

    Ok(())
}

fn wrap_with_prefix(prefix: &str, text: &str, width: usize) -> Vec<String> {
    let prefix_width = prefix.chars().count();
    let available = width.saturating_sub(prefix_width).max(1);
    let wrapped = wrap_text(text, available);
    let indent = " ".repeat(prefix_width);

    wrapped
        .into_iter()
        .enumerate()
        .map(|(idx, segment)| {
            if idx == 0 {
                format!("{}{}", prefix, segment)
            } else {
                format!("{}{}", indent, segment)
            }
        })
        .collect()
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let word_len = word.chars().count();

        if current.is_empty() {
            if word_len <= width {
                current.push_str(word);
            } else {
                let mut chunk = String::new();
                for ch in word.chars() {
                    if chunk.chars().count() >= width {
                        lines.push(chunk);
                        chunk = String::new();
                    }
                    chunk.push(ch);
                }
                if !chunk.is_empty() {
                    current = chunk;
                }
            }
            continue;
        }

        let current_len = current.chars().count();
        if current_len + 1 + word_len <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current);
            current = String::new();

            if word_len <= width {
                current.push_str(word);
            } else {
                let mut chunk = String::new();
                for ch in word.chars() {
                    if chunk.chars().count() >= width {
                        lines.push(chunk);
                        chunk = String::new();
                    }
                    chunk.push(ch);
                }
                current = chunk;
            }
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    if lines.is_empty() {
        vec![String::new()]
    } else {
        lines
    }
}

fn calculate_score(questions: &[Question], answers: &[Option<usize>]) -> Score {
    let mut correct_count = 0usize;
    let mut answered_count = 0usize;

    for (idx, question) in questions.iter().enumerate() {
        if let Some(selected_index) = answers.get(idx).copied().flatten() {
            answered_count += 1;
            if let Some(correct_index) = question.correct_index
                && selected_index == correct_index
            {
                correct_count += 1;
            }
        }
    }

    Score {
        correct_count,
        answered_count,
        total_count: questions.len(),
    }
}

fn persist_csv_files(
    data_dir: &Path,
    questions: &[Question],
    answers: &[Option<usize>],
    score: &Score,
) -> Result<(), String> {
    fs::create_dir_all(data_dir)
        .map_err(|e| format!("Failed to create data dir '{}': {e}", data_dir.display()))?;

    let answer_key_csv = data_dir.join("answer_key.csv");
    let quiz_results_csv = data_dir.join("quiz_results.csv");
    let session_summary_csv = data_dir.join("session_summary.csv");
    let session_id = now_session_id();

    write_answer_key_csv(&answer_key_csv, questions)?;
    append_quiz_results_csv(&quiz_results_csv, questions, answers, &session_id)?;
    append_session_summary_csv(&session_summary_csv, score, &session_id)?;

    Ok(())
}

fn write_answer_key_csv(path: &Path, questions: &[Question]) -> Result<(), String> {
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_path(path)
        .map_err(|e| format!("Failed to create answer key csv '{}': {e}", path.display()))?;

    writer
        .write_record([
            "question_number",
            "module",
            "question",
            "correct_option_number",
            "correct_option_text",
        ])
        .map_err(|e| format!("Failed to write answer key headers: {e}"))?;

    for question in questions {
        let (correct_num, correct_text) = if let Some(correct) = question.correct_index {
            (
                (correct + 1).to_string(),
                question
                    .options
                    .get(correct)
                    .cloned()
                    .unwrap_or_else(String::new),
            )
        } else {
            (String::new(), String::new())
        };

        writer
            .write_record([
                question.number.to_string(),
                question.module.clone(),
                question.text.clone(),
                correct_num,
                correct_text,
            ])
            .map_err(|e| format!("Failed to write answer key row: {e}"))?;
    }

    writer
        .flush()
        .map_err(|e| format!("Failed to flush answer key csv: {e}"))?;

    Ok(())
}

fn append_quiz_results_csv(
    path: &Path,
    questions: &[Question],
    answers: &[Option<usize>],
    session_id: &str,
) -> Result<(), String> {
    let file_exists = path.exists();
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .map_err(|e| format!("Failed to open quiz results csv '{}': {e}", path.display()))?;

    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    if !file_exists {
        writer
            .write_record([
                "session_id",
                "question_number",
                "module",
                "question",
                "selected_option_number",
                "selected_option_text",
                "correct_option_number",
                "correct_option_text",
                "is_correct",
            ])
            .map_err(|e| format!("Failed to write quiz results headers: {e}"))?;
    }

    for (idx, question) in questions.iter().enumerate() {
        let selected_index = answers.get(idx).copied().flatten();
        let (selected_num, selected_text) = if let Some(selected_index) = selected_index {
            (
                (selected_index + 1).to_string(),
                question
                    .options
                    .get(selected_index)
                    .cloned()
                    .unwrap_or_else(String::new),
            )
        } else {
            (String::new(), String::new())
        };

        let (correct_num, correct_text, is_correct) =
            if let Some(correct_index) = question.correct_index {
                (
                    (correct_index + 1).to_string(),
                    question
                        .options
                        .get(correct_index)
                        .cloned()
                        .unwrap_or_else(String::new),
                    selected_index
                        .map(|selected| selected == correct_index)
                        .unwrap_or(false)
                        .to_string(),
                )
            } else {
                (String::new(), String::new(), "false".to_string())
            };

        writer
            .write_record([
                session_id.to_string(),
                question.number.to_string(),
                question.module.clone(),
                question.text.clone(),
                selected_num,
                selected_text,
                correct_num,
                correct_text,
                is_correct,
            ])
            .map_err(|e| format!("Failed to write quiz results row: {e}"))?;
    }

    writer
        .flush()
        .map_err(|e| format!("Failed to flush quiz results csv: {e}"))?;

    Ok(())
}

fn append_session_summary_csv(path: &Path, score: &Score, session_id: &str) -> Result<(), String> {
    let file_exists = path.exists();
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .map_err(|e| {
            format!(
                "Failed to open session summary csv '{}': {e}",
                path.display()
            )
        })?;

    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);

    if !file_exists {
        writer
            .write_record([
                "session_id",
                "timestamp_unix",
                "score",
                "total",
                "percentage",
            ])
            .map_err(|e| format!("Failed to write session summary headers: {e}"))?;
    }

    let percentage = if score.total_count == 0 {
        0.0
    } else {
        (score.correct_count as f64 / score.total_count as f64) * 100.0
    };

    writer
        .write_record([
            session_id.to_string(),
            now_epoch_seconds().to_string(),
            score.correct_count.to_string(),
            score.total_count.to_string(),
            format!("{percentage:.2}"),
        ])
        .map_err(|e| format!("Failed to write session summary row: {e}"))?;

    writer
        .flush()
        .map_err(|e| format!("Failed to flush session summary csv: {e}"))?;

    Ok(())
}

fn now_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn now_session_id() -> String {
    format!("session-{}", now_epoch_seconds())
}

fn print_report(questions: &[Question], answers: &[Option<usize>], score: &Score) {
    let percentage = if score.answered_count == 0 {
        0.0
    } else {
        (score.correct_count as f64 / score.answered_count as f64) * 100.0
    };

    println!("\nQuiz complete.");
    println!(
        "Score so far: {}/{} ({:.2}%)",
        score.correct_count, score.answered_count, percentage
    );
    println!(
        "Questions attempted: {}/{}",
        score.answered_count, score.total_count
    );
    println!("\nDetails:");

    for (idx, question) in questions.iter().enumerate() {
        let selected_index = answers.get(idx).copied().flatten();
        if selected_index.is_none() {
            continue;
        }
        let selected_index = selected_index.unwrap_or(usize::MAX);

        let (selected_num, selected_text) = if selected_index == usize::MAX {
            ("?".to_string(), String::new())
        } else {
            (
                (selected_index + 1).to_string(),
                question
                    .options
                    .get(selected_index)
                    .cloned()
                    .unwrap_or_else(String::new),
            )
        };

        let (correct_num, correct_text, is_correct) =
            if let Some(correct_index) = question.correct_index {
                (
                    (correct_index + 1).to_string(),
                    question
                        .options
                        .get(correct_index)
                        .cloned()
                        .unwrap_or_else(String::new),
                    selected_index == correct_index,
                )
            } else {
                ("?".to_string(), String::new(), false)
            };

        let status = if is_correct { "OK" } else { "WRONG" };
        println!(
            "Q{:02} [{}] Your Answer: {}. {} | Correct: {}. {}",
            question.number, status, selected_num, selected_text, correct_num, correct_text
        );
    }

    println!("\nCSV files: data/answer_key.csv, data/quiz_results.csv, data/session_summary.csv");
}
