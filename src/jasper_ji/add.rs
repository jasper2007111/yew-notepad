use yew::prelude::*;
use yew::{Callback};

use wasm_bindgen::{JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{
     HtmlTextAreaElement,console
};

use yew_router::prelude::*;

use super::repository::Repository;

pub enum Msg {
    
}

pub struct Add {

}

impl Component for Add {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().navigator().unwrap();
        let onclick = Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let _content_element = document
                .get_element_by_id("content")
                .unwrap()
                .dyn_into::<HtmlTextAreaElement>()
                .unwrap();
    
            spawn_local(async move {
                let repository = Repository::new().await;
                // console::log_1(&repository.name.into());
                repository.save(&_content_element.value());
            });
            
            history.back();
        });
        html! {
            <div>
                <h1>{ "Add" }</h1>
                <textarea id="content" placeholder="输入内容"></textarea>
                <div>
                <button {onclick}>{ "提交" }</button>
                </div>
            </div>
        }
    }
}
