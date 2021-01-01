mod composer;

use yew_router::Switch;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/app/main"]
    Main,
}

pub use composer::Composer;
