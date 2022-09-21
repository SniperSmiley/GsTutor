use yew::prelude::*;
use list_component::ListComponent;
use web_sys::{HtmlInputElement,HtmlElement,Element};
use golfscript_rs::golfscript;

mod list_component;


#[function_component(App)]
fn app() -> Html {
    let code = use_state(String::new);
    let input = use_state(String::new);
    let output = use_state(String::new);
    let input_ref_for_code: NodeRef = NodeRef::default();
    let input_ref_for_input: NodeRef = NodeRef::default();
    
    let on_input_for_code = {
        let code = code.clone();
        let input = input.clone();
        let output = output.clone();
        let cur_input = input_ref_for_code.clone();
        Callback::from(move |_e:InputEvent| {
            let input_element = cur_input.cast::<HtmlInputElement>().unwrap();
            let new_code = input_element.value();
            code.set(new_code.clone());
            let golf = golfscript((*input).clone(),new_code);
            output.set(golf);
        })
    };
    let on_input_for_input = {
        let code = code.clone();
        let input = input.clone();
        let output = output.clone();
        let cur_input_for_input = input_ref_for_input.clone();
        Callback::from(move |_e:InputEvent| {
            let input_element = cur_input_for_input.cast::<HtmlInputElement>().unwrap();
            let new_input = input_element.value();
            input.set(new_input.clone());
            if code.is_empty() {
                output.set(String::from(""));
                return;
            }
            let golf = golfscript(new_input,(*code).clone());
            output.set(golf);
        })
    };

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
            <div>
                <textarea class="textarea" ref={input_ref_for_input} oninput={on_input_for_input} placeholder="Input" rows="1"></textarea>
                <textarea class="textarea" ref={input_ref_for_code} oninput={on_input_for_code} placeholder="Code" rows="1"></textarea>

                <p>{(*output).clone()}</p>
            </div>
            <div class="action-container is-basic">
                <div class="action-grid-symbol">
                    <button class="big-button is-option is-mono">{"~"}</button>
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

