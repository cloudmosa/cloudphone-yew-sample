use yew::prelude::*;
use rowdle::{Game, Guess};
use crate::words;
use crate::t9;
use gloo::events::EventListener;
use gloo::timers::callback::Timeout;
use web_sys::KeyboardEvent;
use web_sys::wasm_bindgen::JsCast;
use std::mem;
use crate::views::Board;
use crate::views::Toast;

use super::constants::{WORD_LENGTH, MAX_GUESSES, KEYBOARD_TIMEOUT_MS};

#[derive(PartialEq, Clone)]
pub enum GameStatus {
    Active,
    Won,
    Lost,
}

pub enum GameMessage {
    TypeLetter(char),
    Backspace,
    Submit,
    Escape, /* SoftLeft */
    StartTimer(char),
    TimerFinished,
    RemoveToast(String),
}

pub struct GameState {
    _correct_word: String,
    game: Game<char, String>,
    board_state: Vec<Guess<String, char>>,
    current_guess: String,
    game_status: GameStatus,
    timeout: Option<Timeout>,
    current_letter: Option<char>,
    current_digit: Option<char>,
    toast_message: Option<String>,
}

impl GameState {
    fn with_word(word: String) -> Self {
        let possible_words = include_str!("dictionary.txt").lines().map(ToString::to_string).collect::<Vec<String>>();
        let game = Game::new(MAX_GUESSES as u8, word.clone(), possible_words);
        let board_state = game.board(None, None);

        GameState {
            _correct_word: word,
            game: game,
            board_state: board_state,
            current_guess: String::new(),
            game_status: GameStatus::Active,
            timeout: None,
            current_letter: None,
            current_digit: None,
            toast_message: None,
        }
    }
}

impl Component for GameState {
    type Message = GameMessage;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let (word, _index, _count) = words::fetch_word_of_day();
        Self::with_word(word)
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match (self.game_status == GameStatus::Active, msg) {
            // Add a letter when we haven't reached the word length
            (true, GameMessage::TypeLetter(c)) if self.current_guess.len() < WORD_LENGTH => {
                self.current_guess.push(c.to_ascii_lowercase());
                true
            }
            // Remove the previous letter
            (true, GameMessage::Backspace) => {
                // Clear any timeout, if active
                if let Some(timeout) = self.timeout.take() {
                    timeout.cancel();
                    self.timeout = None;
                    self.current_letter = None;
                }

                self.current_guess.pop();
                true
            }
            // Submit the current word as a guess
            (true, GameMessage::Submit) => {
                if self.current_guess.len() == WORD_LENGTH {
                    let current_guess = mem::take(&mut self.current_guess);
                    let guess_result = self.game.guess(current_guess);

                    match guess_result {
                        Ok(_guess) => {
                            // Check if win or lose
                            if self.game.won() {
                                self.game_status = GameStatus::Won;
                            } else if self.game.lost() {
                                self.game_status = GameStatus::Lost;
                            }

                            self.board_state = self.game.board(None, None);

                            true
                        },
                        Err(error) => {
                            self.toast_message = Some(error.to_string());
                            false
                        }
                    }
                } else {
                    false
                }
            }
            (_, GameMessage::RemoveToast(_message)) => {
                self.toast_message = None;
                true
            }
            (_, GameMessage::Escape) => {
                /* TODO */
                true
            }
            (_, GameMessage::StartTimer(c)) if self.current_guess.len() <= WORD_LENGTH => {
                // Cancel any existing timer
                let has_timer = self.timeout.is_some();
                if let Some(timeout) = self.timeout.take() {
                    timeout.cancel();
                }

                // Set a new timeout
                let link = ctx.link().clone();
                let timeout = Timeout::new(KEYBOARD_TIMEOUT_MS, move || {
                    link.send_message(GameMessage::TimerFinished);
                });

                // Store the timeout
                self.timeout = Some(timeout);

                // Reset loop when we change digits
                if !self.current_digit.eq(&Some(c)) {
                    self.current_digit = Some(c);
                    self.current_letter = None;
                }

                // Grab the next letter in the sequence
                self.current_letter = Some(t9::next_character(self.current_letter.unwrap_or(c)));

                if has_timer {
                    // Replace the last letter in the current guess
                    let guess_len = self.current_guess.len();
                    let new_letter = self.current_letter.unwrap().to_ascii_lowercase().to_string();
                    self.current_guess.replace_range((guess_len-1)..guess_len, &new_letter);
                } else {
                    // Add the letter to the end of the guess
                    self.current_guess.push(self.current_letter.unwrap());
                }

                true // No need to re-render yet
            }
            /*(_, GameMessage::CancelTimer) => {
                if let Some(timeout) = self.timeout.take() {
                    timeout.cancel();
                    self.timeout = None;
                    self.current_letter = None;
                }
                false
            },*/
            (_, GameMessage::TimerFinished) => {
                // Just confirms the current guess
                self.timeout = None;
                self.current_letter = None;

                true // Trigger re-render
            }
            _ => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <Board
                    current_guess={self.current_guess.clone()}
                    board={self.board_state.clone()}
                />

                { match self.toast_message.clone() {
                    Some(message) => html! {
                        <Toast message={message.clone()} on_close={ctx.link().callback(move |_| GameMessage::RemoveToast(message.clone()))} />
                    },
                    _ => html! { }
                }}
            </main>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        /*if self.game_status != GameState::Active {
            self.show_scoreboard(ctx);
        }*/

        let on_keypress: Callback<KeyboardEvent> = ctx.link().batch_callback(handle_keydown);
        let on_back: Callback<Event> = ctx.link().batch_callback(handle_back);

        let document = gloo::utils::document();
        let window = gloo::utils::window();

        EventListener::new(&document, "keydown", move |e: &Event| {
            if let Ok(e) = e.clone().dyn_into::<KeyboardEvent>() {
                on_keypress.emit(e);
            }
        })
        .forget();

        // Custom event for Cloud Phone to handle SoftRight key
        EventListener::new(&window, "back", move |e: &Event| {
            e.prevent_default();
            on_back.emit(e.clone());
        })
        .forget();
    }
}

fn handle_back(_event: Event) -> Option<GameMessage> {
    // Act as the Backspace key
    return Some(GameMessage::Backspace);
}

fn handle_keydown(event: KeyboardEvent) -> Option<GameMessage> {
    if event.key() == "Escape" {
        return Some(GameMessage::Escape);
    }
    if event.key() == "Backspace" {
        return Some(GameMessage::Backspace);
    }
    if event.key() == "Enter" {
        return Some(GameMessage::Submit);
    }
    if event.key().len() > 1 {
        return None;
    }

    if let Some(c) = event.key().chars().next() {
        if c.is_alphabetic() {
            Some(GameMessage::TypeLetter(c))
        } else if c.is_numeric() {
            match c {
                // Ignore 1 in T9
                '2'..'9' => Some(GameMessage::StartTimer(c)),
                _ => None
            }
        } else {
            None
        }
    } else {
        None
    }
}
