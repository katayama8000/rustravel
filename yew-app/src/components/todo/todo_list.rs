use crate::components::todo::todo_item::TodoItem;
use crate::components::todo::types::Todo;
use yew::{function_component, html, Html};

#[function_component(TodoList)]
pub fn todo_list() -> Html {
    let todo = Todo {
        id: 1,
        title: "Learn Rust".to_string(),
        completed: false,
    };
    html! {
      <ul class="list-group">
        <TodoItem title={todo.title} completed={todo.completed} />
      </ul>
    }
}
