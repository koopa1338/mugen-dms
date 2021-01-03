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
        let route = self.route_service.get_route();
        html! {
            <>
                //TODO: put the navigation in own component with property to track active route and
                //add css class to the relevant menu items
                <nav class="uk-navbar-container" uk-navbar="">
                    <div class="uk-navbar-left">
                        <ul class="uk-navbar-nav">
                            <li class="uk-active">
                                <a title="home" href="/app/main">
                                    <span uk-icon="home"></span>
                                </a>
                            </li>
                            <li>
                                <a title="documents" href="/app/docs">
                                    <span uk-icon="album"></span>
                                </a>
                            </li>
                        </ul>
                    </div>
                    <div class="uk-navbar-right">
                        <ul class="uk-navbar-nav">
                            <li>
                                <a title="logout" href="/app/logout">
                                    <span uk-icon="sign-out"></span>
                                </a>
                            </li>
                        </ul>
                    </div>
                </nav>
                <div class="uk-container uk-margin">
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
                <div>{ "Logout route, this should logout and redirect to login page" }</div>
            },
            _ => html! {
                <div>{ "Route not found" }</div>
            },
        }
    }
}

