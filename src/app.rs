use yew::prelude::*;
use reqwasm::http::Request;
use js_sys::JsString;
use web_sys::{
    console,
};
use serde::{Deserialize};
use rand::thread_rng;
use rand::seq::SliceRandom;
use web_sys::HtmlInputElement;

#[derive(Deserialize, Debug)]
struct Country {
    pub name: Name,
    pub flags: Flag,
}

#[derive(Deserialize, Debug)]
struct Name {
    pub common: String,
}

#[derive(Deserialize, Debug)]
struct Flag {
    pub svg: String,
}

// TODO: Requires more error handling -> Result<Vec<Country>, _>
async fn get_countries() -> Vec<Country> {
    let not_permitted_countries: Vec<String> = vec![
        "Guernsey".to_string(),
        "Faroe Islands".to_string(),
        "Jersey".to_string(),
        "Svalbard and Jan Mayen".to_string(),
        "Gibraltar".to_string(),
        "Åland Islands".to_string(),
        "Isle of Man".to_string(),
    ];
    let endpoint = "https://restcountries.com/v3.1/subregion/europe"; 
    let mut resp: Vec<Country> = Request::get(endpoint)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    resp.shuffle(&mut thread_rng());    
    resp.into_iter().filter(|country| !not_permitted_countries.contains(
            &country.name.common)
    ).collect::<Vec<Country>>()
}

#[function_component(App)]
pub fn app() -> Html {
    // Cloning required by Rust's ownership rule
    let resp: UseStateHandle<std::option::Option<Vec<Country>>>  = use_state(|| None);    
    let resp_display = resp.clone();
    let resp_cmp = resp.clone();

    let length = use_state(|| 1);
    let length_clone = length.clone();
    let counter = use_state(|| 0);
    let counter_clone = counter.clone();
    let input_node = use_node_ref();
    let input_node_display = input_node.clone();

    let submit_input = move |e: KeyboardEvent| {
        let resp = resp_cmp.clone();
        let counter = counter_clone.clone();
        let input_node = input_node_display.clone();
        let length = length_clone.clone();

        if e.char_code() == 13 {
            if let Some(input) = input_node.cast::<HtmlInputElement>() {
                match &*resp {
                    Some(countries) => {
                        if countries[*counter].name.common == input.value() && *counter < (*length)-1 {
                            counter.set(*counter + 1);
                        }
                    },
                    None => (),
                }
            }
        }
    };

    {    
        // fetching the data with required metadata for the game
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let countries = get_countries().await;        
                    length.set(countries.len());
                    resp.set(Some(countries)); 
                });
                || ()
            },
            (),
        );
    } 

    html! {
        <main >
            <div>
            {
                match &*resp_display {
                    Some(countries) => {
                        html! {
                            <img src={ countries[*counter].flags.svg.clone() }/>
                        }
                    },
                    None => html!("No data yet"), 
                }
            }
            </div>
            <input ref={input_node} onkeypress={ submit_input }/>
       </main>
    }
}

