use crate::app::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <nav> 
                <ul>
                    <li>
                        <button>
                            <Link<Route> to={Route::Africa}>{ "Africa" }</Link<Route>>
                        </button>
                    </li>
                    <li>
                        <button>
                            <Link<Route> to={Route::America}>{ "America" }</Link<Route>>
                        </button>
                    </li>
                    <li>
                        <button>
                            <Link<Route> to={Route::Asia}>{ "Asia" }</Link<Route>>
                        </button>
                    </li>
                    <li>
                        <button>
                            <Link<Route> to={Route::Europe}>{ "Europe" }</Link<Route>>
                        </button>
                    </li>
                    <li>
                        <button>
                            <Link<Route> to={Route::Oceania}>{ "Oceania" }</Link<Route>>
                        </button>
                    </li>
                </ul>
            </nav> 
        </>
    }
}
