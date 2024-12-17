use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    sign_in_page::SignInPage,sign_up_page::SignUpPage,home_page::HomePage,user_home_page::UserHomePage,chat_room_page::ChatRoomPage
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/signin")]
    SignInPage,
    #[at("/signup")]
    SignUpPage,
    #[at("/user_home/:user_id")]
    UserHomePage { user_id: i32 },
    #[at("/chatroom/:room_id/:user_id")]
    ChatRoomPage { room_id: i32, user_id: i32 },
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage/> },
        Route::SignInPage => html! {<SignInPage/> },
        Route::SignUpPage => html! {<SignUpPage/> },
        Route::UserHomePage {user_id} => html! {<UserHomePage user_id = {user_id}/>},
        Route::ChatRoomPage { room_id, user_id } => html! {
            <ChatRoomPage room_id={room_id} user_id={user_id} />
        },
    }
}
