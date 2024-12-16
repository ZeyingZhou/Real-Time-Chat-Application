use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    chat_room_page::Chat, home_page::HomePage, sign_in_page::SignInPage, sign_up_page::SignUpPage,
    user_home_page::UserHomePage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/signin")]
    SignInPage,
    #[at("/signup")]
    SignUpPage,
    #[at("/user")]
    UserHomePage,
    #[at("/chatroom/:room_id/:user_id")]
    Chat { room_id: String, user_id: String },
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage/> },
        Route::SignInPage => html! {<SignInPage/> },
        Route::SignUpPage => html! {<SignUpPage/> },
        Route::UserHomePage => html! {<UserHomePage/> },
        Route::Chat { room_id, user_id } => html! {
            <Chat/>
        },
    }
}
