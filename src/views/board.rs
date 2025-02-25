use yew::prelude::*;
use rowdle::Guess;

use super::row::{Row, from_guess};
use super::LetterValue;
use super::super::constants::{WORD_LENGTH, MAX_GUESSES};

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub current_guess: String,
    pub board: Vec<Guess<String, char>>,
}

#[function_component(Board)]
pub fn view(props: &BoardProps) -> Html {
    // Always display a fixed grid
    let mut guessed_rows: Vec<Vec<LetterValue>> = props.board.clone().into_iter()
        .map(|guess| from_guess(guess)).collect();
    let mut rows = vec![vec![LetterValue::Empty; WORD_LENGTH]; MAX_GUESSES];

    // Fill in the current, unsubmitted guess
    if guessed_rows.len() < rows.len() {
        let mut guess_row = vec![LetterValue::Typing(' '); WORD_LENGTH];
        for (idx, c) in props.current_guess.chars().enumerate() {
            guess_row[idx] = LetterValue::Typing(c);
        }
        guessed_rows.push(guess_row);
    }

    // Fill in each row from the list of guesses
    for (i, val) in guessed_rows.into_iter().enumerate() {
        rows[i] = val;
    }

    html! {
        <div class="grid board">
            {
                rows.into_iter()
                    .map(|r| {
                        html! { <Row values={r} /> }
                    }).collect::<Html>()
            }
        </div>
    }
}
