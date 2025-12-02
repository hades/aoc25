use crate::aocclient::ValidationResult;

use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::thread::sleep;

use log::debug;
use serde::Deserialize;
use serde::Serialize;

const FILE: &str = "results.toml";

#[derive(Debug, Default, Deserialize, Serialize)]
struct PuzzleLogEntry {
    rejected_answers: Vec<String>,
    accepted_answer: Option<String>,
    upper_bound: Option<i64>,
    lower_bound: Option<i64>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Log {
    answers: HashMap<String, PuzzleLogEntry>,
}

fn puzzle_key(day: i8, level: i8) -> String {
    format!("day{0}part{1}", day, level)
}

fn read_submission_log() -> Option<Log> {
    let log = fs::read_to_string(FILE).ok()?;
    let log: Log = toml::from_str(log.as_str()).ok()?;
    Some(log)
}

fn write_submission_log(log: &Log) {
    fs::write(FILE, toml::to_string_pretty(log).unwrap().as_str()).unwrap();
}

fn check_submission_log(day: i8, level: i8, answer: &str) -> Option<ValidationResult> {
    let log = read_submission_log()?;
    if let Some(entry) = log.answers.get(puzzle_key(day, level).as_str()) {
        if let Some(accepted_answer) = &entry.accepted_answer {
            return Some(if answer == accepted_answer {
                ValidationResult::Accepted
            } else {
                ValidationResult::Rejected
            });
        }
        if let Ok(answer_int) = answer.parse::<i64>() {
            if let Some(upper_bound) = entry.upper_bound
                && answer_int >= upper_bound
            {
                return Some(ValidationResult::RejectedTooHigh);
            }
            if let Some(lower_bound) = entry.lower_bound
                && answer_int <= lower_bound
            {
                return Some(ValidationResult::RejectedTooLow);
            }
        }
        if entry.rejected_answers.iter().any(|a| a == answer) {
            return Some(ValidationResult::Rejected);
        }
    }
    None
}

fn record_submission_log(day: i8, level: i8, answer: &str, result: &ValidationResult) {
    let mut log = read_submission_log()
        .or_else(|| Some(Log::default()))
        .unwrap();
    let key = puzzle_key(day, level);
    if !log.answers.contains_key(&key) {
        log.answers.insert(key.clone(), PuzzleLogEntry::default());
    }
    let entry = log.answers.get_mut(&key).unwrap();
    match *result {
        ValidationResult::Accepted => {
            entry.accepted_answer = Some(answer.to_string());
        }
        ValidationResult::Rejected => {
            if !entry.rejected_answers.iter().any(|a| a == answer) {
                entry.rejected_answers.push(answer.to_string());
            }
        }
        ValidationResult::RejectedTooLow => {
            if let Ok(answer_int) = answer.parse::<i64>() {
                let mut lower_bound = answer_int;
                if let Some(old_lower_bound) = entry.lower_bound {
                    lower_bound = cmp::max(lower_bound, old_lower_bound);
                }
                entry.lower_bound = Some(lower_bound);
            }
        }
        ValidationResult::RejectedTooHigh => {
            if let Ok(answer_int) = answer.parse::<i64>() {
                let mut upper_bound = answer_int;
                if let Some(old_upper_bound) = entry.upper_bound {
                    upper_bound = cmp::min(upper_bound, old_upper_bound);
                }
                entry.upper_bound = Some(upper_bound);
            }
        }
        ValidationResult::Throttled(_) => {
            panic!("unexpected Throttled value in record_submission_log");
        }
    }
    write_submission_log(&log);
}

pub fn submit_with_cache<'a, F>(
    day: i8,
    level: i8,
    answer: &'a str,
    mut submit_fn: F,
) -> ValidationResult
where
    F: FnMut(i8, i8, &'a str) -> ValidationResult,
{
    if let Some(result) = check_submission_log(day, level, answer) {
        debug!("answer provided by submission log in results.toml");
        return result;
    }
    if answer.is_empty() || answer == "0" {
        debug!("cowardly refusing to submit the answer of {answer}");
        return ValidationResult::Rejected;
    }
    let mut result;
    loop {
        result = submit_fn(day, level, answer);
        if let ValidationResult::Throttled(timeout) = result {
            sleep(timeout);
        } else {
            break;
        }
    }
    record_submission_log(day, level, answer, &result);
    result
}

pub fn next_unsolved_day() -> i8 {
    let mut last_fully_solved_day = 0;
    if let Some(submission_log) = read_submission_log() {
        for day in 1..24 {
            if let (Some(part1), Some(part2)) = (
                submission_log.answers.get(puzzle_key(day, 1).as_str()),
                submission_log.answers.get(puzzle_key(day, 2).as_str()),
            ) && part1.accepted_answer.is_some()
                && part2.accepted_answer.is_some()
            {
                last_fully_solved_day = day;
            }
        }
    }
    last_fully_solved_day + 1
}
