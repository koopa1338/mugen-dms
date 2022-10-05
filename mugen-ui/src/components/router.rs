use yew_router::Routable;

#[derive(Clone, Eq, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/app")]
    Main,
    #[at("/app/docs")]
    Docs,
    #[not_found]
    #[at("/404")]
    NotFound,
}
