mod app;

// use app::App;
use app::BeeQueenApp as App;

fn main() {
    yew::Renderer::<App>::new().render();
}
