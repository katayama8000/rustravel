// use components::todo::types::Todo;
use yew::{function_component, html, Html, Properties}; // Properties を yew から持ち込む

#[derive(Properties, PartialEq)] // Properties, PartialEq を継承した構造体を作成
pub struct TodoItemProps {
    pub title: String,
    pub completed: bool,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    html! {
      <li class="list-group-item">
      <input class="form-check-input me-2" type="checkbox" checked={props.completed} />
        {&props.title}
      </li>
    }
}
