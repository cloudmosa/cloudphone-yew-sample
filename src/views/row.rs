use yew::prelude::*;
use rowdle::Guess;

use super::{Letter, LetterValue};

#[derive(Properties, PartialEq)]
pub struct RowProps {
    pub values: Vec<LetterValue>,
}

pub struct Row;

pub fn from_guess(guess: Guess<String, char>) -> Vec<LetterValue> {
    guess.guess.into_iter().map(|x| x.into()).collect()
}

impl Component for Row {
    type Message = ();

    type Properties = RowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="row">
                { ctx.props().values.clone().iter().map(|c| html! { <Letter value={c} /> }).collect::<Html>() }
            </div>
        }
    }
}
