use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tokio::task::spawn_local;
use uuid::Uuid;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};
#[derive(Serialize, Deserialize)]
struct UuidResponse {
    uuid: Uuid,
}

async fn fetch_uuid() -> Uuid {
    // reqwest works for both non-wasm and wasm targets.
    let resp = reqwest::get("https://httpbin.org/uuid").await.unwrap();
    let uuid_resp = resp.json::<UuidResponse>().await.unwrap();

    uuid_resp.uuid
}

pub struct UuidState {
    s: Suspension,
    value: Rc<RefCell<Option<Uuid>>>,
}

impl UuidState {
    fn new() -> Self {
        let (s, handle) = Suspension::new();
        let value: Rc<RefCell<Option<Uuid>>> = Rc::default();

        {
            let value = value.clone();
            // we use tokio spawn local here.
            spawn_local(async move {
                let uuid = fetch_uuid().await;

                {
                    let mut value = value.borrow_mut();
                    *value = Some(uuid);
                }

                handle.resume();
            });
        }

        Self { s, value }
    }
}

impl PartialEq for UuidState {
    fn eq(&self, rhs: &Self) -> bool {
        self.s == rhs.s
    }
}

#[hook]
fn use_random_uuid() -> SuspensionResult<Uuid> {
    let s = use_state(UuidState::new);

    let result = match *s.value.borrow() {
        Some(ref m) => Ok(*m),
        None => Err(s.s.clone()),
    };

    result
}

#[function_component]
fn Content() -> HtmlResult {
    let uuid = use_random_uuid()?;

    Ok(html! {
        <div>{"Random UUID: "}{uuid}</div>
    })
}

#[function_component]
fn App() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense {fallback}>
            <Content />
        </Suspense>
    }
}

#[derive(Properties, PartialEq)]
pub struct HelloProps {
    pub name: String,
}

#[function_component]
pub fn Hello(props: &HelloProps) -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div>
            <App/>
            <button {onclick}>{ "Increment value" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
            <p>
              { props.name.clone() }
            </p>
        </div>
    }
}
