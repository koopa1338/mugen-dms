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
            <div>{"Mugen DMS"}</div>
        }
    }
}

impl Composer {
    fn view_main_pane(&self) -> Html {
        let route = self.route_service.get_route();
        match AppRoute::switch(route) {
            Some(AppRoute::Main) => html! {
                <div>{"MAIN"}</div>
            },
            _ => html! {
                <div>{ "Route not found" }</div>
            },
        }
    }
}

