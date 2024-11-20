use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    signInPage::SignInPage,signUpPage::SignUpPage,homePage::HomePage
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/signin")]
    SignInPage,
    #[at("/signup")]
    SignUpPage,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<HomePage/> },
        Route::SignInPage => html! {<SignInPage/> },
        Route::SignUpPage => html! {<SignUpPage/> },
    }
}