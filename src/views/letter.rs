use yew::{html::ImplicitClone, prelude::*};
use rowdle::GuessResult;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LetterValue {
    Empty,
    Typing(char),
    FoundExactMatch(char),
    FoundInWord(char),
    NotFound(char)
}

impl ImplicitClone for LetterValue {}

impl From<GuessResult<char>> for LetterValue {
    fn from(guess_result: GuessResult<char>) -> Self {
        match guess_result {
            GuessResult::Empty => LetterValue::Empty,
            GuessResult::Correct(v) => LetterValue::FoundExactMatch(v),
            GuessResult::Incorrect(v) => LetterValue::NotFound(v),
            GuessResult::Misplaced(v) => LetterValue::FoundInWord(v),
            GuessResult::Custom(v) => LetterValue::Typing(v)
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct LetterProps {
    pub value: LetterValue,
}

#[function_component(Letter)]
pub fn cell(props: &LetterProps) -> Html {
    match props.value {
        LetterValue::Empty => {
            html! {
                <div class="empty letter" />
            }
        }
        LetterValue::Typing(v) => {
            html! {
                <div class="empty letter">{v}</div>
            }
        }
        LetterValue::FoundExactMatch(v) => {
            html! {
                <div class="found-exact-match letter">{v}</div>
            }
        }
        LetterValue::FoundInWord(v) => {
            html! {
                <div class="found-in-word letter">{v}</div>
            }
        }
        LetterValue::NotFound(v) => {
            html! {
                <div class="not-found letter">{v}</div>
            }
        }
    }
}
