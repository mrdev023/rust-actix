use yew::prelude::*;

use crate::models::user::User;

#[derive(Properties, PartialEq)]
pub struct UserComponentProps {
    pub user: User,
}

#[function_component]
pub fn UserComponent(props: &UserComponentProps) -> Html {
  html! {
      <div>
        <p>{ props.user.clone().id } </p>
        <p>{ props.user.clone().name } </p>
      </div>
  }
}