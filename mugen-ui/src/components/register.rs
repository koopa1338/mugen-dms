use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Register {}

impl Component for Register {
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
            <div class="uk-container uk-width-large" id="login-component">
                <div class="uk-card-default uk-card-body uk-align-center">
                    <h3 class="uk-card-title">{"Register"}</h3>
                    <form>
                        <div class="uk-margin">
                            <div class="uk-inline uk-width-1-1">
                                <span class="uk-form-icon" uk-icon="icon: mail"></span>
                                <input class="uk-input" type="text" name="email" />
                            </div>
                        </div>

                        <div class="uk-margin">
                            <div class="uk-inline uk-width-1-1">
                                <span class="uk-form-icon" uk-icon="icon: user"></span>
                                <input class="uk-input" type="text" name="username"/>
                            </div>
                        </div>

                        <div class="uk-margin">
                            <div class="uk-inline uk-width-1-1">
                                <span class="uk-form-icon" uk-icon="icon: lock"></span>
                                <input class="uk-input" type="password" name="password" />
                            </div>
                        </div>

                        <div class="uk-margin">
                            <div class="uk-inline uk-width-1-1">
                                <button class="uk-button uk-button-primary uk-width-1-1">{"Register"}</button>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        }
    }
}
