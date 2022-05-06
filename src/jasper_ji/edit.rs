use yew::prelude::*;
use yew::Callback;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, HtmlInputElement, HtmlTextAreaElement};

use yew_router::prelude::*;

use super::fetch_error::FetchError;
use super::note::Note;
use super::repository::Repository;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: u32,
}

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

pub enum Msg {
    SetTodoFetchState(FetchState<Note>),
    GetTodo,
}

pub struct Edit {
    id: u32,
    note: FetchState<Note>,
}

async fn fetch_todo(id: u32) -> Result<Note, FetchError> {
    let repository = Repository::new().await;
    let note = repository.getNote(id).await;

    Ok(note)
}

impl Component for Edit {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        console::log_1(&ctx.props().id.clone().into());
        let _self = Self {
            note: FetchState::NotFetching,
            id: ctx.props().id.clone(),
        };
        // _self.note = FetchState::Fetching;
        let id = ctx.props().id.clone();
        ctx.link().send_future(async move {
            match fetch_todo(id).await {
                Ok(md) => Msg::SetTodoFetchState(FetchState::Success(md)),
                Err(err) => Msg::SetTodoFetchState(FetchState::Failed(err)),
            }
        });
        ctx.link()
            .send_message(Msg::SetTodoFetchState(FetchState::Fetching));
        _self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let id = self.id.clone();
        match msg {
            Msg::SetTodoFetchState(fetch_state) => {
                self.note = fetch_state;
                true
            }
            Msg::GetTodo => {
                ctx.link().send_future(async move {
                    match fetch_todo(id).await {
                        Ok(md) => Msg::SetTodoFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetTodoFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetTodoFetchState(FetchState::Fetching));
                false
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().navigator().unwrap();
        let onclick = Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let content_element = document
                .get_element_by_id("content")
                .unwrap()
                .dyn_into::<HtmlTextAreaElement>()
                .unwrap();
            let content = content_element.value().clone();

            let id_element = document
                .get_element_by_id("noteId")
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let id_value = id_element.value().clone();

            let create_time_element = document
                .get_element_by_id("createTime")
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let create_time_value = create_time_element.value().clone();

            spawn_local(async move {
                let repository = Repository::new().await;
                let note = Note {
                    id: id_value.parse::<u32>().unwrap(),
                    content: content,
                    create_time: create_time_value
                };
                repository.putNote(&note);
            });

            history.back();
        });

        match &self.note {
            FetchState::NotFetching => html! {
                <div>{"NotFetching"}</div>
            },
            FetchState::Fetching => html! {
                <div>{"Fetching"}</div>
            },
            FetchState::Success(note) => html! {
                <div>
                <h1>{ "Edit" }</h1>
                <input value={note.id.clone().to_string()} type="hidden" id={"noteId"}/>
                <input value={note.create_time.clone()} type="hidden" id={"createTime"}/>
                <textarea id={"content"} placeholder={"输入内容"} value={note.content.clone()}></textarea>
                <div>
                <button {onclick}>{ "更新" }</button>
                </div>
            </div>
            },
            FetchState::Failed(err) => html! { err },
        }
    }
}
