use crate::game::GameDisplay;
use crate::home::Home;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
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
    #[at("/oceania")]
    Oceania,
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <BrowserRouter>
                <header>
                    <Link<Route> to={Route::Home}><h1> { "Flag Quiz" } </h1></Link<Route>>
                </header>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </main>
    }

}
fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Europe => html! { <GameDisplay region={"https://restcountries.com/v3.1/region/europe"}/> },
        Route::Asia => html! { <GameDisplay region={"https://restcountries.com/v3.1/region/asia"}/> },
        Route::Africa => html! { <GameDisplay region={"https://restcountries.com/v3.1/region/africa"}/> },
        Route::America=> html! { <GameDisplay region={"https://restcountries.com/v3.1/region/ame"}/> },
        Route::Oceania=> html! { <GameDisplay region={"https://restcountries.com/v3.1/region/Oceania"}/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
