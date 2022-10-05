use super::navigation::Menu;
use super::router::AppRoute;

use yew::{html, Component, Context, Html};
use yew_router::{BrowserRouter, Switch};

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <Menu/>
                <div class="uk-container uk-margin uk-align-center">
                    <BrowserRouter>
                        <Switch<AppRoute> render={Switch::render(switch)} />
                    </BrowserRouter>
                </div>
            </div>
        }
    }
}

fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Main => html! {
            <div class="uk-card-default uk-card-body">
                <h3 class="uk-card-title">{"Home"}</h3>
                <p>{"This is the main route"}</p>
            </div>
        },
        AppRoute::Docs => html! {
            <div class="uk-card-default uk-card-body">
                <h3 class="uk-card-title">{"Docs"}</h3>
                <p>{"This is the docs route"}</p>
            </div>
        },
        AppRoute::NotFound => html! {
            {"Route not found!"}
        },
    }
}
