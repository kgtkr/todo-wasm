#[macro_use]
extern crate yew;
use yew::prelude::*;

type Context = ();

struct Model {
    todos: Vec<(i32, String, bool)>,
    filter: bool,
    form: String,
    next_id: i32,
}

enum Msg {
    Add(String),
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
            Msg::Add(name) => {
                self.todos.push((self.next_id, name, false));
                self.next_id += 1;
            }
            Msg::Remove(id) => {
                self.todos = self.todos.into_iter().filter(|(x, ..)| *x != id).collect();
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

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <>
                <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
                {if self.click{
                    "サンキュークリック"
                }else{
                    "クリックしてね"
                }}
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
