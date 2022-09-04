use yew::prelude::*;
use list_component::ListComponent;

mod list_component;

#[function_component(App)]
fn app() -> Html {
    html! {
    <body>
        <div class="sidenav">
            <div>
                <a>{"Fibbinacci"}</a>
            </div>
            <div>
                <button class="sidebutton">{"Squares"}</button>
            </div>
        </div>
        <div class="main">
            <textarea class="textarea" placeholder="e.g. Hello world" rows="1"></textarea>
            <div class="action-container is-basic">
                <div class="action-grid-symbol">
                    <button class="big-button is-option is-family-monospace">{"~"}</button>
                </div>
                <div class="action-grid-access1">
                    <button class="button is-option">{"ℤ"}</button>
                    <button class="button is-blackened">{"\"\""}</button>
                    <button class="button is-blackened">{"{}"}</button>
                    <button class="button is-blackened">{"[]"}</button>
                    {"bitwise NOT"}
                </div>
                <div class="action-grid-access2">
                    <button class="button is-blackened">{"ℤ"}</button>
                    <button class="button is-option">{"\"\""}</button>
                    <button class="button is-option">{"{}"}</button>
                    <button class="button is-blackened">{"[]"}</button>
                    {"Evaluate"}
                </div>
                <div class="action-grid-access3">
                    <button class="button is-blackened">{"ℤ"}</button>
                    <button class="button is-blackened">{"\"\""}</button>
                    <button class="button is-blackened">{"{}"}</button>
                    <button class="button is-option">{"[]"}</button>
                    {"Dump"}            
                </div>
                <div class="action-grid-script">{"lala"}</div>
            </div>
            <div>
                
            </div>
            <div>
            </div>
            <button class="big-button is-invalid">{"`"}</button>
        </div>
    </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("App is starting");
    yew::start_app::<App>();
}

