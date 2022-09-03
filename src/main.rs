use yew::prelude::*;
use list_component::ListComponent;

mod list_component;

#[function_component(App)]
fn app() -> Html {
    html! {
    <body class="main">
        <textarea class="textarea" placeholder="e.g. Hello world" rows="1"></textarea>
        <div>
            <button class="big-button is-success is-family-monospace">{"~"}</button>
        </div>
        <div>
            <div>
                <div class="field is-grouped">
                    <button class="button is-option">{"ℤ"}</button>
                    <button class="button is-blackened">{"\"\""}</button>
                    <button class="button is-blackened">{"{}"}</button>
                    <button class="button is-blackened">{"[]"}</button>
                    {"bitwise NOT"}
                </div>
                <div class="field is-grouped">
                    <button class="button is-blackened">{"ℤ"}</button>
                    <button class="button is-option">{"\"\""}</button>
                    <button class="button is-option">{"{}"}</button>
                    <button class="button is-blackened">{"[]"}</button>
                    {"Evaluate"}
                </div>
                <div class="field is-grouped">
                    <button class="button is-blackened">{"ℤ"}</button>
                    <button class="button is-blackened">{"\"\""}</button>
                    <button class="button is-blackened">{"{}"}</button>
                    <button class="button is-option">{"[]"}</button>
                    {"Dump"}
                </div>
            </div>
        </div>
        <button class="big-button is-option">{"`"}</button>
    </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("App is starting");
    yew::start_app::<App>();
}

