use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::atoms::button::CustomButton;

use crate::MIDIKeyboardProperties;

#[derive(PartialEq)]
pub enum KeyColor {
    White,
    Black
}

impl KeyColor {
    pub fn to_string(&self) -> String {
        match self {
            Self::White => "whitekey".to_owned(),
            Self::Black => "blackkey".to_owned(),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct KeyProps {
    pub label: char,
    pub key_color: KeyColor,
    pub on_mouse_down: Callback<char>,
    pub on_mouse_up: Callback<char>,
}

#[styled_component(Key)]
pub fn key(props: &KeyProps) -> Html {
    let mouse_down = props.on_mouse_down.clone();
    let label = props.label.clone();
    let mouse_down = Callback::from(move |_| {
        mouse_down.emit(label);
    });

    let mouse_up = props.on_mouse_up.clone();
    let label = props.label.clone();
    let mouse_up = Callback::from(move |_| {
        mouse_up.emit(label);
    });

    html! {
        <div class = {&props.key_color.to_string()}>
            <CustomButton 
                class="keycodes" 
                label={props.label.to_string()} 
                is_active={false}
                mouse_down={mouse_down}
                mouse_up={mouse_up}
            />
        </div>
    }
}


pub fn create_white_keys(props: &MIDIKeyboardProperties) -> Vec<Html> {
    let mut keys = Vec::new();
    let keycodes = vec!['A', 'S', 'D', 'F', 'G', 'H', 'J', 'K'];

    for index in 0..8{
        let mouse_down = props.mouse_down.clone();
        let mouse_up = props.mouse_up.clone();
        let mouse_down = Callback::from(move |label: char| {
            mouse_down.emit(label)
        });
        let mouse_up = Callback::from(move |label: char| {
            mouse_up.emit(label)
        });
        keys.push(html! {
            <Key 
                label={&keycodes[index]} 
                key_color={KeyColor::White} 
                on_mouse_down={&mouse_down} 
                on_mouse_up= {&mouse_up} 
            />
        })
    }
    keys
}