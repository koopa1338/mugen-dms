use yew_router::Switch;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/app"]
    Root,
    #[to = "/app/main"]
    Main,
    #[to = "/app/docs"]
    Docs,
}
