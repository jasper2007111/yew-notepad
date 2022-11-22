use super::note::Note;

use futures_channel::oneshot;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    console, IdbCursorWithValue, IdbDatabase, IdbObjectStoreParameters, IdbRequest,
    IdbTransactionMode,IdbKeyRange
};
use yew::prelude::*;

use chrono::prelude::*;

const DATE_FORMAT_STR: &'static str = "%Y-%m-%d %H:%M:%S";
const DB_NAME: &'static str = "notepad";

pub struct Repository {
    db: IdbDatabase,
}

impl Repository {
    pub async fn new() -> Repository {
        let (tx, rx) = oneshot::channel::<IdbDatabase>();
        let window = web_sys::window().unwrap();
        let idb_factory = window.indexed_db().unwrap().unwrap();

        let open_request = idb_factory.open_with_u32(DB_NAME, 1).unwrap();

        let on_upgradeneeded = Closure::once(move |event: &Event| {
            let target = event.target().expect("Event should have a target; qed");
            let req = target
                .dyn_ref::<IdbRequest>()
                .expect("Event target is IdbRequest; qed");

            let result = req
                .result()
                .expect("IndexedDB.onsuccess should have a valid result; qed");
            assert!(result.is_instance_of::<IdbDatabase>());
            let db = IdbDatabase::from(result);
            // let _store: IdbObjectStore = db.create_object_store(&String::from("user")).unwrap();
            let mut parameters: IdbObjectStoreParameters = IdbObjectStoreParameters::new();
            parameters.auto_increment(true);
            parameters.key_path(Some(&JsValue::from_str(String::from("id").as_str())));
            let _store =
                db.create_object_store_with_optional_parameters(&String::from("note"), &parameters);
            // let _index = store
            //     .create_index_with_str(&String::from("name"), &String::from("name"))
            //     .expect("create_index_with_str error");
        });
        open_request.set_onupgradeneeded(Some(on_upgradeneeded.as_ref().unchecked_ref()));
        on_upgradeneeded.forget();

        let on_success = Closure::once(move |event: &Event| {
            // Extract database handle from the event
            let target = event.target().expect("Event should have a target; qed");
            let req = target
                .dyn_ref::<IdbRequest>()
                .expect("Event target is IdbRequest; qed");

            let result = req
                .result()
                .expect("IndexedDB.onsuccess should have a valid result; qed");
            assert!(result.is_instance_of::<IdbDatabase>());

            let db = IdbDatabase::from(result);
            let _ = tx.send(db);
        });
        open_request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
        on_success.forget();

        let db = rx.await.unwrap();
        Repository { db }
    }

    pub async fn get_note(&self, id: u32) -> Note {
        let (tx, rx) = oneshot::channel::<Note>();

        let transaction = self
            .db
            .transaction_with_str_and_mode(&String::from("note"), IdbTransactionMode::Readwrite)
            .expect("transaction_with_str error");
        let store = transaction
            .object_store(&String::from("note"))
            .expect("store error");

        console::log_1(&id.clone().to_string().into());

        let request = store.get(&JsValue::from(id.clone())).expect("get all error");
        let on_add_error = Closure::once(move |event: &Event| {
            console::log_1(&String::from("读取数据失败").into());
            console::log_1(&event.into());
        });
        request.set_onerror(Some(on_add_error.as_ref().unchecked_ref()));
        on_add_error.forget();

        let on_success = Closure::once(move |event: &Event| {
            let target = event.target().expect("msg");
            let req = target
                .dyn_ref::<IdbRequest>()
                .expect("Event target is IdbRequest; qed");
            let result = req.result().expect("read result error");
            console::log_1(&result.clone().into());
            let note: Note = result.into_serde().expect("msg");
            console::log_1(&note.content.clone().into());
            // console::log_1(&String::from("读取数据成功").into());
            let _ = tx.send(note);
        });
        request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
        on_success.forget();

        let note = rx.await.unwrap();
        note
    }

    pub fn put_note(&self, note: &Note)  {
        // let (tx, rx) = oneshot::channel::<Note>();
        console::log_1(&String::from("dddddd").into());

        let transaction = self
            .db
            .transaction_with_str_and_mode(&String::from("note"), IdbTransactionMode::Readwrite)
            .expect("transaction_with_str error");
        let store = transaction
            .object_store(&String::from("note"))
            .expect("store error");

        let request = store.put(&JsValue::from_serde(&note).unwrap()).expect("get all error");
        let on_add_error = Closure::once(move |event: &Event| {
            console::log_1(&String::from("更新数据失败").into());
            console::log_1(&event.into());
        });
        request.set_onerror(Some(on_add_error.as_ref().unchecked_ref()));
        on_add_error.forget();

        let on_success = Closure::once(move |event: &Event| {
            let target = event.target().expect("msg");
            let req = target
                .dyn_ref::<IdbRequest>()
                .expect("Event target is IdbRequest; qed");
            let result = req.result().expect("read result error");
            console::log_1(&result.clone().into());
            console::log_1(&String::from("更新数据成功").into());
            // let _ = tx.send(note);
        });
        request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
        on_success.forget();

        // let note = rx.await.unwrap();
        // note
    }

    pub fn delete_note(&self, id: u32)  {
        // let (tx, rx) = oneshot::channel::<Note>();
        console::log_1(&String::from("dddddd").into());

        let transaction = self
            .db
            .transaction_with_str_and_mode(&String::from("note"), IdbTransactionMode::Readwrite)
            .expect("transaction_with_str error");
        let store = transaction
            .object_store(&String::from("note"))
            .expect("store error");

        let request = store.delete(&JsValue::from(id)).expect("get all error");
        let on_add_error = Closure::once(move |event: &Event| {
            console::log_1(&String::from("删除数据失败").into());
            console::log_1(&event.into());
        });
        request.set_onerror(Some(on_add_error.as_ref().unchecked_ref()));
        on_add_error.forget();

        let on_success = Closure::once(move |event: &Event| {
            let target = event.target().expect("msg");
            let req = target
                .dyn_ref::<IdbRequest>()
                .expect("Event target is IdbRequest; qed");
            let result = req.result().expect("read result error");
            console::log_1(&result.clone().into());
            console::log_1(&String::from("删除数据成功").into());
            // let _ = tx.send(note);
        });
        request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
        on_success.forget();

        // let note = rx.await.unwrap();
        // note
    }

    pub async fn list(&self) -> Vec<Note> {
        let (tx, rx) = oneshot::channel::<Vec<Note>>();

        let transaction = self
            .db
            .transaction_with_str_and_mode(&String::from("note"), IdbTransactionMode::Readwrite)
            .expect("transaction_with_str error");
        let store = transaction
            .object_store(&String::from("note"))
            .expect("store error");

        let rang = IdbKeyRange::bound(&JsValue::from(0), &JsValue::from(100));
        let request = store.open_cursor_with_range_and_direction(&JsValue::from(&rang.unwrap()), web_sys::IdbCursorDirection::Prev).unwrap();
        let on_add_error = Closure::once(move |event: &Event| {
            console::log_1(&String::from("读取数据失败").into());
            console::log_1(&event.into());
        });
        request.set_onerror(Some(on_add_error.as_ref().unchecked_ref()));
        on_add_error.forget();

        let mut todo_list = Vec::new();
        let mut tx = Some(tx);
        let on_success = Closure::wrap(Box::new(move |event: &Event| {
            let target = event.target().expect("msg");
            let req = target
                .dyn_ref::<IdbRequest>()
                .expect("Event target is IdbRequest; qed");
            let result = match req.result() {
                Ok(data) => data,
                Err(err) => JsValue::null(),
            };
            // let todo_list_ref = Rc::clone(&todo_list);
            if !result.is_null() {
                // console::log_1(&result.clone().into());
                let db_cursor_with_value = result
                    .dyn_ref::<IdbCursorWithValue>()
                    .expect("db_cursor_with_value error");
                let value = db_cursor_with_value.value().expect("value error");
                let note: Note = value.into_serde().expect("msg");
                todo_list.push(note);
                let _ = db_cursor_with_value.continue_();

                // console::log_1(&(*todo_list_ref).borrow_mut().len().into());
            } else {
                let _ = tx.take().unwrap().send(todo_list.to_owned());
            }
        }) as Box<dyn FnMut(&Event)>);
        request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
        on_success.forget();

        let list = rx.await.unwrap();
        list
    }

    pub fn save(&self, str: &String) {
        let transaction = self
            .db
            .transaction_with_str_and_mode(&String::from("note"), IdbTransactionMode::Readwrite)
            .expect("transaction_with_str error");
        let store = transaction
            .object_store(&String::from("note"))
            .expect("store error");

        let now = js_sys::Date::new_0();
        let dt = DateTime::<Utc>::from(now.clone()); // 表示只在这个里面实现了

        let note = Note {
            id: None,
            content: str.clone(),
            create_time: dt.format(DATE_FORMAT_STR).to_string(),
        };
        let add_request = store.add(&JsValue::from_serde(&note).unwrap()).expect(&str);

        let on_add_error = Closure::once(move |event: &Event| {
            console::log_1(&String::from("写入数据失败").into());
            console::log_1(&event.into());
        });
        add_request.set_onerror(Some(on_add_error.as_ref().unchecked_ref()));
        on_add_error.forget();

        let on_add_success = Closure::once(move |event: &Event| {
            console::log_1(&String::from("写入数据成功").into());
        });
        add_request.set_onsuccess(Some(on_add_success.as_ref().unchecked_ref()));
        on_add_success.forget();

        console::log_1(&String::from("do").into());
    }
}
