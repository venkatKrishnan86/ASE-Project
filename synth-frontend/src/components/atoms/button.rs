use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub class: String,
    pub label: String,
    pub is_active: bool,
    pub mouse_down: Callback<MouseEvent>,
    pub mouse_up: Callback<MouseEvent>
}

#[styled_component(CustomButton)]
pub fn custom_button(props: &ButtonProps) -> Html{
    html! {
        <button 
            class = {&props.class} 
            onmousedown={&props.mouse_down} 
            onmouseup={&props.mouse_up}
        >{&props.label}</button>
    }
}