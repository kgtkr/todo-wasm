#![feature(drain_filter)]
#![feature(nll)]

#[macro_use]
extern crate yew;
use yew::prelude::*;
#[macro_use]
extern crate stdweb;
type Context = ();

struct Model {
    todos: Vec<(i32, String, bool)>,
    filter: bool,
    form: String,
    next_id: i32,
}

enum Msg {
    Add,
    Remove(i32),
    SwitchFilter,
    ChangeForm(String),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            todos: Vec::new(),
            filter: false,
            form: "".to_string(),
            next_id: 0,
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.todos.push((self.next_id, self.form.clone(), false));
                self.form = "".to_string();
                self.next_id += 1;
            }
            Msg::Remove(id) => {
                self.todos.drain_filter(|(x, ..)| *x == id);
            }
            Msg::SwitchFilter => {
                self.filter = !self.filter;
            }
            Msg::ChangeForm(text) => {
                self.form = text;
            }
        }
        true
    }
}

fn todo((id, name, _): &(i32, String, bool)) -> Html<Context, Model> {
    let id = *id;
    html!{
        <li><button onclick=move|_| Msg::Remove(id), >{"削除"}</button>{name}</li>
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <>
                <div>
                    <input value=&self.form, oninput=|e:yew::html::InputData|Msg::ChangeForm(e.value),/>
                    <button onclick=|_| Msg::Add, >{"追加"}</button>
                </div>
                <ul>
                    {for self.todos.iter().map(todo)}
                </ul>
            </>
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
