use yew::{html, Component, Context, Html};

pub struct Menu {}

impl Component for Menu {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="uk-navbar-container" uk-navbar="">
                <div class="uk-navbar-left">
                    <ul class="uk-navbar-nav">
                        <li class="uk-active">
                            <a title="home" href="/app">
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
            </nav>
        }
    }
}
