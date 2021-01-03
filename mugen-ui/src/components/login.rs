use yew::{Component, ComponentLink, Html, ShouldRender};

pub struct Login {
}

impl Component for Login {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        unimplemented!();
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!();
    }

    fn change(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!();
    }

    fn view(&self) -> Html {
        unimplemented!();
    }
}
