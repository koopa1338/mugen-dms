use super::content::Content;

use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Composer {}

impl Component for Composer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_main_pane() }
            </>
        }
    }
}

impl Composer {
    fn view_main_pane(&self) -> Html {
        html! {
            <Content/>
        }
    }
}
