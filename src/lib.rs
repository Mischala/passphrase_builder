#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::{Html};

mod word;

pub struct Model {
    words: Vec<word::Word>,
}

#[derive(Debug, Clone)]
pub enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            words: vec![
                word::Word::random_word().unwrap(),
                word::Word::random_word().unwrap(),
                word::Word::random_word().unwrap()
                ]
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

fn word_view(word: &word::Word) -> Html<Model> {
    html!{
            <div class="word",>
                { word.to_string() }
            </div>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="words",>
                { for self.words.iter().map(word_view) }
            </div>
        }
    }
}