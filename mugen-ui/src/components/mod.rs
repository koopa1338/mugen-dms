mod composer;
mod login;
mod navigation;

use yew_router::Switch;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/app/main"]
    Main,
    #[to = "/app/docs"]
    Docs,
    #[to = "/app/logout"]
    Logout,
}

pub use composer::Composer;
pub use login::Login;
pub use navigation::MainNavigation;
