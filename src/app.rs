use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::layout::Layout;
use crate::pages::articles;
use crate::pages::index;

use super::router::AppRoute;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Layout>
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Home => html!{<index::Index />},
                            AppRoute::Articles => html! {<articles::index::Index />},
                            AppRoute::Article(id) => html!{<articles::article::Article id=id />},
                        }
                    })
                />
          </Layout>
        }
    }
}
