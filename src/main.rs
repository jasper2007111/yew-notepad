use yew::prelude::*;
use yew_router::prelude::*;

mod jasper_ji;
use jasper_ji::add::Add;
use jasper_ji::home::Home;
use jasper_ji::route::Route;
use jasper_ji::edit::Edit;

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Add => html! {
            <Add/>
        },
        Route::Edit{id} => html! {
            <Edit id={id}/>
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
