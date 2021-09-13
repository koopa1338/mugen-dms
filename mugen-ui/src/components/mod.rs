mod composer;
mod content;
mod login;
mod navigation;
mod register;

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
    #[to = "/app/register"]
    Register,
}

pub use composer::Composer;
pub use content::Content;
pub use login::Login;
pub use navigation::MainNavigation;
pub use register::Register;
