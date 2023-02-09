use std::rc::Rc;

use yew::{function_component, html, ChildrenWithProps, Component, Context, Html, Properties};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MenuProps {
    pub children: ChildrenWithProps<NavLink>,
}

pub struct Menu {}

impl Component for Menu {
    type Message = ();
    type Properties = MenuProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <nav class="bg-white border-gray-200 px-2 sm:px-4 py-2.5 dark:bg-gray-900">
                <div class="container flex flex-wrap justify-between items-center mx-auto">
                    <div class="hidden w-full md:block md:w-auto" id="navbar-default">
                        <ul class="flex flex-col p-4 mt-4 bg-gray-50 rounded-lg border border-gray-100 md:flex-row md:space-x-8 md:mt-0 md:text-sm md:font-medium md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
                            {
                                for ctx.props().children.iter().map(|mut item| {
                                    let mut props = Rc::make_mut(&mut item.props);
                                    props.url = format!("/app/{}", props.url);
                                    item
                                })
                            }
                        </ul>
                    </div>
                </div>
            </nav>
        }
    }
}

#[derive(Clone, Properties, PartialEq, Eq)]
pub struct NavLinkProps {
    pub title: String,
    pub url: String,
}

#[function_component(NavLink)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    let props = props.clone();
    html! {
         <li>
           <a href={props.url} class="block py-2 pr-4 pl-3 text-gray-700 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-gray-400 md:dark:hover:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">{props.title}</a>
         </li>
    }
}
