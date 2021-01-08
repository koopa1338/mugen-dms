use super::login::Login;
use super::navigation::MainNavigation;
use super::AppRoute;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{service::RouteService, Switch};

pub struct Composer {
    route_service: RouteService<()>,
}

impl Component for Composer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            route_service: RouteService::new(),
        }
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
                //TODO: if we are not authenticated don't render the navigation
                <MainNavigation/>
                <div class="uk-container uk-margin uk-align-center">
                    { self.view_main_pane() }
                </div>
            </>
        }
    }
}

impl Composer {
    fn view_main_pane(&self) -> Html {
        let route = self.route_service.get_route();
        match AppRoute::switch(route) {
            Some(AppRoute::Main) => html! {
                <div class="uk-card-default uk-card-body">
                    <h3 class="uk-card-title">{"Home"}</h3>
                    <p >{"This is the main route"}</p>
                </div>
            },
            Some(AppRoute::Docs) => html! {
                <div class="uk-card-default uk-card-body">
                    <h3 class="uk-card-title">{"Docs"}</h3>
                    <p>{"This is the docs route"}</p>
                </div>
            },
            Some(AppRoute::Logout) => html! {
                <Login/>
            },
            Some(AppRoute::Login) => html! {
                <Login/>
            },
            _ => html! {
                <div>{ "Route not found" }</div>
            },
        }
    }
}
