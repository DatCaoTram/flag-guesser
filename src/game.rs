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
async fn get_countries(continent: &str) -> Vec<Country> {
    // This either due to obscurity or 
    let omitted_flags: Vec<String> = vec![
        // Europe
        "Guernsey".to_string(),
        "Faroe Islands".to_string(),
        "Jersey".to_string(),
        "Svalbard and Jan Mayen".to_string(),
        "Gibraltar".to_string(),
        // Africa
        "British Indian Ocean Territory".to_string(),
        "Réunion".to_string(),
        "São Tomé and Príncipe".to_string(),
        "Saint Helena, Ascension and Tristan da Cunha".to_string(),
        "Mayotte".to_string(),
        // America
        "Aruba".to_string(),
        "Bonaire".to_string(),
        "Clipperton Island".to_string(),
        "Curaçao".to_string(),
        "French Guiana".to_string(),
        "Greenland".to_string(),
        "Guadeloupe".to_string(),
        "Martinique".to_string(),
        "Saba".to_string(),
        "Saint Barthélemy".to_string(),
        "Saint Martin".to_string(),
        "Saint Pierre and Miquelon".to_string(),
        "Sint Eustatius".to_string(),
        "Sint Maarten".to_string(),
        "Anguilla".to_string(),
        "Bermuda".to_string(),
        "British Virgin Islands".to_string(),
        "Cayman Islands".to_string(),
        "Caribbean Netherlands".to_string(),
        "Falkland Islands".to_string(),
        "Montserrat".to_string(),
        "Navassa Island".to_string(),
        "Puerto Rico".to_string(),
        "Turks and Caicos Islands".to_string(),
        "United States Virgin Islands".to_string(),
        "United States Minor Outlying Islands".to_string(),
    ];
    let mut resp: Vec<Country> = Request::get(continent)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    resp.shuffle(&mut thread_rng());    
    resp.into_iter().filter(|country| !omitted_flags.contains(
            &country.name.common)
    ).collect::<Vec<Country>>()
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub region: String,
}

#[function_component(GameDisplay)]
pub fn game_display(props: &Props) -> Html {
    // Cloning required by Rust's ownership rule
    let resp: UseStateHandle<std::option::Option<Vec<Country>>>  = use_state(|| None);    
    let resp_display = resp.clone();
    let resp_cmp = resp.clone();

    let length = use_state(|| 0);
    let length_clone = length.clone();
    let counter = use_state(|| 0);
    let counter_clone = counter.clone();
    let input_node = use_node_ref();
    let input_node_display = input_node.clone();

    let correct_answer = use_state(|| String::new()); 
    let correct_answer_store = correct_answer.clone();

    let submit_input = move |e: KeyboardEvent| {
        let resp = resp_cmp.clone();
        let counter = counter_clone.clone();
        let input_node = input_node_display.clone();
        let length = length_clone.clone();

        if e.char_code() == 13 {
            if let Some(input) = input_node.cast::<HtmlInputElement>() {
                match &*resp {
                    Some(countries) => {
                        if countries[*counter].name.common == input.value() {
                            counter.set(*counter + 1);
                            input.set_value("");
                        } else {
                            counter.set(*counter + 1);
                        }
                    },
                    None => (),
                }
            }
        }
    };

    {    
        let region = props.region.clone();
        // fetching the data with required metadata for the game
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let countries = get_countries(&region).await;        
                    length.set(countries.len());
                    console::log_1(&JsString::from(format!("{}", countries.len())));
                    resp.set(Some(countries)); 
                });
                || ()
            },
            (),
        );
    } 

    html! {
        <>
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
        </>
    }
}
