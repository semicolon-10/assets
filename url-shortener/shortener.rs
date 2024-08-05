use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;



#[function_component(UrlShortener)]
pub fn url_shortener() -> Html {

    html! {
        <div style="font-family: Arial, sans-serif; background-color: #f0f0f0; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0;">
            <div style="background-color: #ffffff; padding: 20px; border-radius: 8px; box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1); text-align: center; width: 300px;">
                <h1 style="font-size: 24px; margin-bottom: 20px;">{ "URL Shortener" }</h1>
                <input
                    type="text"
                    placeholder="Enter long URL"
                    value={(*long_url).clone()}
                    oninput={oninput}
                    style="width: calc(100% - 22px); padding: 10px; margin-bottom: 10px; border: 1px solid #ccc; border-radius: 4px;"
                />
                <button onclick={shorten_url} style="background-color: #007bff; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer; margin: 5px;">
                    { "Shorten" }
                </button>
                { if let Some(ref url) = *short_url {
                    html! {
                        <div style="display: flex; flex-direction: column; align-items: center; margin-top: 20px;">
                            <input type="text" readonly=true value={url.clone()} style="width: 100%; padding: 10px; border: 1px solid #ccc; border-radius: 4px; margin-bottom: 10px; text-align: center; font-size: 16px;" />
                            <button onclick={open_in_new_tab} style="background-color: #28a745; color: white; border: none; padding: 10px 20px; border-radius: 4px; cursor: pointer; margin: 5px;">
                                { "Open in New Tab" }
                            </button>
                        </div>
                    }
                } else {
                    html! { <div></div> }
                }}
            </div>
        </div>
    }
}
