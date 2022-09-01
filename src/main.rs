use yew::prelude::*;
use list_component::ListComponent;

mod list_component;

#[function_component(App)]
fn app() -> Html {
    html! {
    <body>
        <textarea class="textarea" placeholder="e.g. Hello world" rows="1"></textarea>
        <div class="box">
            <button class="button is-large is-success is-family-monospace">{"~"}</button>
            <div class="field is-grouped">
                <button class="button is-small is-success is-family-monospace">{"ℤ"}</button>
                <button class="button is-small is-outlined is-family-monospace">{"\"\""}</button>
                <button class="button is-small is-outlined is-family-monospace">{"{}"}</button>
                <button class="button is-small is-outlined is-family-monospace">{"[]"}</button>
                {"bitwise NOT"}
            </div>
            <div class="field is-grouped">
                <button class="button is-small is-outlined is-family-monospace">{"ℤ"}</button>
                <button class="button is-small is-success is-family-monospace">{"\"\""}</button>
                <button class="button is-small is-success is-family-monospace">{"{}"}</button>
                <button class="button is-small is-outlined is-family-monospace">{"[]"}</button>
                {"Evaluate"}
            </div>
            <div class="field is-grouped">
                <button class="button is-small is-outlined is-family-monospace">{"ℤ"}</button>
                <button class="button is-small is-outlined is-family-monospace">{"\"\""}</button>
                <button class="button is-small is-outlined is-family-monospace">{"{}"}</button>
                <button class="button is-small is-success is-family-monospace">{"[]"}</button>
                {"Dump"}
            </div>
        </div>

    </body>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("App is starting");
    yew::start_app::<App>();
}

