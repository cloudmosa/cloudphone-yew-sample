use chrono::Local;

use crate::dictionary::DICTIONARY;

const ONE_DAY_IN_MS: u64 = 86_400_000;
const START_DATE: u64 = 1740308400000; // 2/24/2025

pub fn get_today_word_index() -> u64 {
    let game_started_at = START_DATE;
    let now = current_timestamp();
    (now - game_started_at) / ONE_DAY_IN_MS
}

fn current_timestamp() -> u64 {
    Local::now().timestamp_millis() as u64
}

pub fn get_word_of_day(dictionary: &[&str], word_index: Option<u64>) -> (String, u64, usize) {
    let words_count = dictionary.len();
    let today_index = get_today_word_index();
    let index = word_index.unwrap_or(today_index) as usize;
    let word = dictionary[index % words_count].to_string();
    (word, index as u64, words_count)
}

pub fn fetch_word_of_day() -> (String, u64, usize) {
    let words: &[&str] = &DICTIONARY;
    get_word_of_day(words, None)
}
