use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Login {}

impl Component for Login {
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
                    <h3 class="uk-card-title">{"Login"}</h3>
                    <form>
                        <div class="uk-margin">
                            <div class="uk-inline uk-width-1-1">
                                <span class="uk-form-icon" uk-icon="icon: user"></span>
                                <input class="uk-input" type="text" />
                            </div>
                        </div>

                        <div class="uk-margin">
                            <div class="uk-inline uk-width-1-1">
                                <span class="uk-form-icon" uk-icon="icon: lock"></span>
                                <input class="uk-input" type="password" />
                            </div>
                        </div>
                        <div class="uk-margin uk-grid-small uk-child-width-auto uk-grid">
                            <label><input class="uk-checkbox uk-margin-right" type="checkbox" />{"Keep me logged in"}</label>
                        </div>
                        <div class="uk-margin">
                            <div class="uk-inline uk-width-1-1">
                                <button class="uk-button uk-button-primary uk-width-1-1">{"Login"}</button>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        }
    }
}
