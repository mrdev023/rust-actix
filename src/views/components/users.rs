use yew::prelude::*;

use crate::models::user::User;
use crate::views::components::user::UserComponent;

#[derive(Properties, PartialEq)]
pub struct UsersComponentProps {
    pub users: Vec<User>,
}

#[function_component]
pub fn UsersComponent(props: &UsersComponentProps) -> Html {
  let users = props.users.clone().iter().map(|user| html! {
    <UserComponent user={user.clone()} />
  }).collect::<Html>();
  html! {
      <div>
        { users }
      </div>
  }
}