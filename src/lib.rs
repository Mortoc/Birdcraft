use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);
    let mut dark_mode = use_signal(|| false);

    let bg_color = if dark_mode() { "#1a1a1a" } else { "#ffffff" };
    let text_color = if dark_mode() { "#e0e0e0" } else { "#2c3e50" };
    let card_bg = if dark_mode() { "#2d2d2d" } else { "#f8f9fa" };
    let button_bg = if dark_mode() { "#4a4a4a" } else { "#f0f0f0" };
    let counter_bg = if dark_mode() { "#3a3a3a" } else { "#e8f4fd" };

    rsx! {
        div {
            style: "
                min-height: 100vh;
                background: {bg_color};
                color: {text_color};
                text-align: center; 
                padding: 20px; 
                font-family: Arial, sans-serif;
                transition: all 0.3s ease;
            ",
            
            div {
                style: "
                    max-width: 400px;
                    margin: 0 auto;
                    background: {card_bg};
                    padding: 30px;
                    border-radius: 15px;
                    box-shadow: 0 4px 15px rgba(0,0,0,0.1);
                ",
                
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                    h1 { 
                        style: "margin: 0; color: {text_color};",
                        "🐦 Birdcraft" 
                    }
                    button {
                        style: "
                            background: {button_bg};
                            color: {text_color};
                            border: none;
                            padding: 8px 12px;
                            border-radius: 8px;
                            cursor: pointer;
                            font-size: 14px;
                            transition: all 0.3s ease;
                        ",
                        onclick: move |_| dark_mode.toggle(),
                        if dark_mode() { "☀️ Light" } else { "🌙 Dark" }
                    }
                }
                
                p { 
                    style: "color: {text_color}; margin-bottom: 30px;",
                    "Welcome to your Dioxus mobile app!" 
                }
                
                div {
                    style: "margin: 20px 0;",
                    button {
                        style: "
                            background: #3498db; 
                            color: white; 
                            border: none; 
                            padding: 12px 24px; 
                            border-radius: 8px;
                            margin: 5px;
                            font-size: 16px;
                            cursor: pointer;
                            transition: all 0.3s ease;
                        ",
                        onclick: move |_| count += 1,
                        "Click me! +"
                    }
                    
                    button {
                        style: "
                            background: #e74c3c; 
                            color: white; 
                            border: none; 
                            padding: 12px 24px; 
                            border-radius: 8px;
                            margin: 5px;
                            font-size: 16px;
                            cursor: pointer;
                            transition: all 0.3s ease;
                        ",
                        onclick: move |_| count -= 1,
                        "Decrease -"
                    }
                }
                
                div {
                    style: "
                        background: {counter_bg};
                        padding: 20px;
                        border-radius: 10px;
                        margin-top: 20px;
                    ",
                    p {
                        style: "
                            font-size: 24px; 
                            color: {text_color};
                            margin: 0;
                            font-weight: bold;
                        ",
                        "Count: {count}"
                    }
                }
            }
        }
    }
}