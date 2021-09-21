use super::navigation::MainNavigation;
use super::router::AppRoute;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use yew_router::{service::RouteService, Switch};

pub struct Content {
    route_service: RouteService<()>,
}

impl Component for Content {
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
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <MainNavigation/>
                <div class="uk-container uk-margin uk-align-center">
                    { self.view_content() }
                </div>
            </>
        }
    }
}

impl Content {
    fn view_content(&self) -> Html {
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
            _ => html! {
                {"Route not found!"}
            },
        }
    }
}
