#![feature(drain_filter)]
#![feature(nll)]

extern crate serde;
#[macro_use]
extern crate yew;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::*;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
type Context = ();

struct Model {
    storage: StorageService,
    data: Data,
}

#[derive(Serialize, Deserialize)]
struct Data {
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
    Completion(i32),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        let mut storage = StorageService::new(Area::Local);
        let data = if let Json(Ok(data)) = storage.restore("todo") {
            data
        } else {
            Data {
                todos: Vec::new(),
                filter: false,
                form: "".to_string(),
                next_id: 0,
            }
        };
        Model {
            storage: storage,
            data: data,
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.data
                    .todos
                    .push((self.data.next_id, self.data.form.clone(), false));
                self.data.form = "".to_string();
                self.data.next_id += 1;
            }
            Msg::SwitchFilter => {
                self.data.filter = !self.data.filter;
            }
            Msg::Remove(id) => {
                self.data.todos.drain_filter(|(x, ..)| *x == id);
            }
            Msg::ChangeForm(text) => {
                self.data.form = text;
            }
            Msg::Completion(id) => {
                for x in &mut self.data.todos {
                    if x.0 == id {
                        x.2 = !x.2;
                        break;
                    }
                }
            }
        }
        self.storage.store("todo", Json(&self.data));
        true
    }
}

fn todo((id, name, completion): &(i32, String, bool)) -> Html<Context, Model> {
    let id = *id;
    let completion = *completion;
    html!{
        <li><button onclick=move|_| Msg::Completion(id), >{if completion {"完了"}else {"未完了"}}</button><button onclick=move|_| Msg::Remove(id), >{"削除"}</button>{name}</li>
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <>
                <div>
                    <input value=&self.data.form, oninput=|e:yew::html::InputData|Msg::ChangeForm(e.value),/>
                    <button onclick=|_| Msg::Add, >{"追加"}</button>
                </div>
                <div>
                    <button onclick=|_| Msg::SwitchFilter, >{if self.data.filter {"未完了のみ"} else {"全て"}}</button>
                </div>
                <ul>
                    {for self.data.todos.iter().filter(|x|!self.data.filter||!x.2).map(todo)}
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
