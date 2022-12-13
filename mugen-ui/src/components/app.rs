use super::navigation::Menu;
use super::navigation::NavLink;
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

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="bg-white dark:bg-gray-900">
                <Menu>
                    <NavLink title="Home" url="" />
                    <NavLink title="Docs" url="docs" />
                </Menu>
                <div class="bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-400">
                    <div class="py-8 px-4 mx-auto max-w-screen-xl sm:py-16 lg:px-6">
                        <div class="max-w-screen-md">
                            <BrowserRouter>
                                <Switch<AppRoute> render={switch} />
                            </BrowserRouter>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Main => html! {
            <div>
                <h3 class="">{"Home"}</h3>
                <p>{"This is the main route"}</p>
            </div>
        },
        AppRoute::Docs => html! {
            <div>
                <h3 class="">{"Docs"}</h3>
                <p>{"This is the docs route"}</p>
            </div>
        },
        AppRoute::NotFound => html! {
            <h3 class="">{"Route not found!"}</h3>
        },
    }
}
