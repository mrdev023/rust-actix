use yew::prelude::*;

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