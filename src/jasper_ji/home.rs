use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

use yew::prelude::*;
use yew::Callback;
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;

use super::note::Note;
use super::repository::Repository;
use super::route::Route;

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: String,
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}
impl Error for FetchError {}

impl From<String> for FetchError {
    fn from(value: String) -> Self {
        Self { err: value }
    }
}

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
                        <div>
                        {
                            data.into_iter().map(|note| {
                                html!{
                                    <div style="margin: 10px 10px 0 0;">
                                    <div>{note.content.clone()}</div>
                                    <div>{note.create_time.clone()}</div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }

                        </div>
                        <div style="margin: 10px 10px 0 0;"><button {onclick}>{ "添加" }</button></div>
                    </div>
                }
            }(),
            FetchState::Failed(err) => html! { err },
        }
    }
}
