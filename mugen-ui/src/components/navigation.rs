use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct MainNavigation {}

impl Component for MainNavigation {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
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
        }
    }
}
