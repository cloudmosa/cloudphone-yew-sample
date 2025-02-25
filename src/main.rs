mod app;
mod words;
mod dictionary;
mod t9;
mod views;
mod constants;

use app::GameState;

fn main() {
    yew::Renderer::<GameState>::new().render();
}
