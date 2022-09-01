use web_sys::{HtmlInputElement};
use yew::prelude::*;

#[function_component(ListComponent)]
pub fn list_component() -> Html {
    let name_list = use_state(Vec::new);
    let input_ref: NodeRef = NodeRef::default();
    let code_input = use_state(String::new);
    let code_text = String::from("");

    let on_input = {
        let code = code_input.clone();
        let cur_input = input_ref.clone();
        Callback::from(move |_e:InputEvent| {
            let mut names = (*code).clone();
            let name_input_element = cur_input.cast::<HtmlInputElement>().unwrap();
            let new_name = name_input_element.value();
            names = new_name;
            code.set(names);
        })
    };

    let on_click = {
        let name_list = name_list.clone();
        let cur_input = input_ref.clone();
        Callback::from(move |_e:MouseEvent| {
            let mut names = (*name_list).clone();
            let name_input_element = cur_input.cast::<HtmlInputElement>().unwrap();
            let new_name = name_input_element.value();
            name_input_element.set_value("");
            names.push(new_name);
            name_list.set(names);

        })
    };

    let display_names = (*name_list).iter().map(|name| html! {<li>{name}</li>});
    html! {
        <div class="bd-link">
            <input class="title" ref={input_ref} oninput={on_input} type="text" placeholder="input a name"/>
            <div class="bd-vars">
                <div class="bd-var bd-is-body">{(*code_input).clone()}</div>
            </div>
        </div>
    }
}