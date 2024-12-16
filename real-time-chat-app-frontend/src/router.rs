use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    sign_in_page::SignInPage,sign_up_page::SignUpPage,home_page::HomePage,profile_page::ProfilePage,user_home_page::UserHomePage,chat_room_page::ChatRoomPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/signin")]
    SignInPage,
    #[at("/signup")]
    SignUpPage,
    #[at("/profile")]
    ProfilePage,
    #[at("/user")]
    UserHomePage,
    #[at("/chat")] 
    ChatRoom
    
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage/> },
        Route::SignInPage => html! {<SignInPage/> },
        Route::SignUpPage => html! {<SignUpPage/> },
        Route::ProfilePage => html! {<ProfilePage/> },
        Route::ChatRoom => html! { <ChatRoomPage /> }, 
        Route::UserHomePage => html! {<UserHomePage/>}
    }
}