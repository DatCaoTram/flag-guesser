use crate::game::GameDisplay;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/europe")]
    Europe,
    #[at("/asia")]
    Asia,
    #[at("/africa")]
    Africa,
    #[at("/america")]
    America,
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </main>
    }

}
fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Link<Route> to={Route::Europe}>{ "click here to go europe" }</Link<Route>> },
        Route::Europe => html! { <GameDisplay region={"https://restcountries.com/v3.1/region/europe"}/> },
        Route::Asia => html! { <GameDisplay region={"https://restcountries.com/v3.1/region/asia"}/> },
        Route::Africa => html! { <GameDisplay region={"https://restcountries.com/v3.1/region/africa"}/> },
        Route::America=> html! { <GameDisplay region={"https://restcountries.com/v3.1/region/ame"}/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
