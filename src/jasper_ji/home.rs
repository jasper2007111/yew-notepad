
use web_sys::console;
use yew::prelude::*;
use yew::Callback;
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

use yew::html::Scope;

use std::{cell::RefCell, rc::Rc};

use super::note::Note;
use super::repository::Repository;
use super::route::Route;
use super::fetch_error::FetchError;



pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

async fn fetch_todo() -> Result<Vec<Note>, FetchError> {
    let repository = Repository::new().await;
    let todo_list = repository.list().await;

    Ok(todo_list)
}

pub enum Msg {
    SetEdit(u32),
    SetTodoFetchState(FetchState<Vec<Note>>),
    GetTodo,
    GetError,
}

pub struct Home {
    todo: FetchState<Vec<Note>>,
}

impl Home {
    pub fn send_get_todo_msg(&self, ctx: &Context<Self>) {
        ctx.link().send_future(async {
            match fetch_todo().await {
                Ok(md) => Msg::SetTodoFetchState(FetchState::Success(md)),
                Err(err) => Msg::SetTodoFetchState(FetchState::Failed(err)),
            }
        });
        ctx.link()
            .send_message(Msg::SetTodoFetchState(FetchState::Fetching));
    }
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let home = Self {
            todo: FetchState::NotFetching,
        };

        home.send_get_todo_msg(ctx);
        return home;
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetTodoFetchState(fetch_state) => {
                self.todo = fetch_state;
                true
            }
            Msg::GetTodo => {
                ctx.link().send_future(async {
                    match fetch_todo().await {
                        Ok(md) => Msg::SetTodoFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetTodoFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetTodoFetchState(FetchState::Fetching));
                false
            }
            Msg::GetError => {
                ctx.link().send_future(async {
                    match fetch_todo().await {
                        Ok(md) => Msg::SetTodoFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetTodoFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetTodoFetchState(FetchState::Fetching));
                false
            }
            Msg::SetEdit(id) => {
                console::log_1(&id.into());
                let history1 = ctx.link().navigator().unwrap();
                history1.push(Route::Edit{id:id.clone()});
                false
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().navigator().unwrap();
        let onclick = Callback::from(move |_| history.push(Route::Add));

        match &self.todo {
            FetchState::NotFetching => html! {
                <>
                    <button onclick={ctx.link().callback(|_| Msg::GetTodo)}>
                        { "Get Markdown" }
                    </button>
                    <button onclick={ctx.link().callback(|_| Msg::GetError)}>
                        { "Get using incorrect URL" }
                    </button>
                </>
            },
            FetchState::Fetching => html! { "Fetching" },
            FetchState::Success(data) => || -> Html {
                html! {
                    <div>
                        <h1>{ "记事本" }</h1>
                        <div style="margin: 10px 10px 0 0;"><button {onclick}>{ "添加" }</button></div>
                        <div>
                        {
                            data.into_iter().map(|note| {
                                let id = note.id.unwrap();
                                html!{
                                    <div style="margin: 10px 10px 0 0; display: flex; flex-direction: column;">
                                    <div>{note.content.clone()}</div>
                                    <div style="margin: 10px 0px 0px 0px; display: flex; flex-direction: row;">
                                    <div>{note.create_time.clone()}</div>
                                    <button style="margin: 0px 0px 0px 10px; " onclick={ctx.link().callback(move|_|Msg::SetEdit(id))}>{"编辑"}</button>
                                    </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                        </div>
                    </div>
                }
            }(),
            FetchState::Failed(err) => html! { err },
        }
    }
}
