#[macro_use]
extern crate yew;
use yew::prelude::*;

type Context = ();

struct Model {
    click: bool,
}

enum Msg {
    DoIt,
}

impl Component<Context> for Model {
    // Some details omitted. Explore the examples to get more.

    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model { click: false }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                self.click = true;
                true
            }
        }
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
