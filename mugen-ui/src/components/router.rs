use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/app")]
    Main,
    #[at("/app/docs")]
    Docs,
    #[not_found]
    #[at("/404")]
    NotFound,
}
