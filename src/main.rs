use dioxus::prelude::*;

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            style: "text-align: center; padding: 20px; font-family: Arial, sans-serif;",
            h1 { "🐦 Birdcraft" }
            p { "Welcome to your Dioxus mobile app!" }
            
            div {
                style: "margin: 20px 0;",
                button {
                    style: "
                        background: #3498db; 
                        color: white; 
                        border: none; 
                        padding: 10px 20px; 
                        border-radius: 5px;
                        margin: 5px;
                        font-size: 16px;
                    ",
                    onclick: move |_| count += 1,
                    "Click me!"
                }
                
                button {
                    style: "
                        background: #e74c3c; 
                        color: white; 
                        border: none; 
                        padding: 10px 20px; 
                        border-radius: 5px;
                        margin: 5px;
                        font-size: 16px;
                    ",
                    onclick: move |_| count -= 1,
                    "Decrease"
                }
            }
            
            p {
                style: "font-size: 18px; color: #2c3e50;",
                "Count: {count}"
            }
        }
    }
}
