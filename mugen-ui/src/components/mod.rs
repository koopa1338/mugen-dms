mod composer;
mod content;
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
    #[to = "/app/login"]
    Login,
}

pub use composer::Composer;
pub use login::Login;
pub use navigation::MainNavigation;
pub use content::Content;
