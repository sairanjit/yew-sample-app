use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component]
pub fn App() -> Html {
    let name = "Sai Ranjit";
    let my_object = MyObject {
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };

    log!("name is", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let class = "my_title";

    let message: Option<&str> = None;

    let tasks = vec!["Learn Rust", "Learn Yew", "Build a cool web app"];

    html! {
      <>
        <h1 class={class}>{"Hello World!!"}</h1>
        if class == "my_title" {
          <p>{"Hi There"}</p>
        } else {
          <p>{"Bye There"}</p>
        }

        if let Some(message) = message {
          <p>{message}</p>
        } else {
          <p>{"No messages"}</p>
        }

        <ul>
          {list_to_html(tasks)}
        </ul>

      </>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|item| html! {<li>{item}</li>}).collect()
}
