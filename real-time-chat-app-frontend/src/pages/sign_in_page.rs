use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::api::user_api::api_signin_user;
use crate::components::{form_input::FormInput, loading_button::LoadingButton};
use crate::router::{self, Route};
use crate::store::{set_page_loading, set_show_alert, Store};

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]

struct SignInUserSchema {
    #[validate(
        length(min = 1, message = "User Name is required"),
    )]
    username: String,
    #[validate(
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    password: String,
}

fn get_input_callback(
    name: &'static str,
    cloned_form: UseStateHandle<SignInUserSchema>,
) -> Callback<String> {
    Callback::from(move |value| {
        let mut data = cloned_form.deref().clone();
        match name {
            "username" => data.username = value,
            "password" => data.password = value,
            _ => (),
        }
        cloned_form.set(data);
    })
}

#[function_component(SignInPage)]
pub fn sign_in_page() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let form = use_state(|| SignInUserSchema::default());
    let validation_errors = use_state(|| Rc::new(RefCell::new(ValidationErrors::new())));
    let navigator = use_navigator().unwrap();

    let username_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();

    let validate_input_on_blur = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        Callback::from(move |(name, value): (String, String)| {
            let mut data = cloned_form.deref().clone();
            match name.as_str() {
                "username" => data.username = value,
                "password" => data.password = value,
                _ => (),
            }
            cloned_form.set(data);

            match cloned_form.validate() {
                Ok(_) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .remove(name.as_str());
                }
                Err(errors) => {
                    cloned_validation_errors
                        .borrow_mut()
                        .errors_mut()
                        .retain(|key, _| key != &name);
                    for (field_name, error) in errors.errors() {
                        if field_name == &name {
                            cloned_validation_errors
                                .borrow_mut()
                                .errors_mut()
                                .insert(field_name.clone(), error.clone());
                        }
                    }
                }
            }
        })
    };

    let handle_username_input = get_input_callback("username", form.clone());
    let handle_password_input = get_input_callback("password", form.clone());

    let on_submit = {
        let cloned_form = form.clone();
        let cloned_validation_errors = validation_errors.clone();
        let store_dispatch = dispatch.clone();
        let cloned_navigator = navigator.clone();

        let cloned_username_input_ref = username_input_ref.clone();
        let cloned_password_input_ref = password_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let dispatch = store_dispatch.clone();
            let form = cloned_form.clone();
            let validation_errors = cloned_validation_errors.clone();
            let navigator = cloned_navigator.clone();

            let username_input_ref = cloned_username_input_ref.clone();
            let password_input_ref = cloned_password_input_ref.clone();

            spawn_local(async move {
                match form.validate() {
                    Ok(_) => {
                        let form_data = form.deref().clone();
                        set_page_loading(true, dispatch.clone());

                        let username_input = username_input_ref.cast::<HtmlInputElement>().unwrap();
                        let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();

                        username_input.set_value("");
                        password_input.set_value("");
    
                        let form_json = serde_json::to_string(&form_data).unwrap();
                        let res = api_signin_user(&form_json).await;
                        match res {
                            Ok(signin_response) => {
         
                                let user_id = signin_response.user_id;
                                web_sys::window()
                                    .unwrap()
                                    .local_storage()
                                    .unwrap()
                                    .unwrap()
                                    .set_item("user_id", &signin_response.user_id.to_string())
                                    .expect("Failed to store user ID in localStorage");
    
                                set_page_loading(false, dispatch);
                                navigator.push(&router::Route::UserHomePage { user_id });
                            }
                            Err(e) => {
                                set_page_loading(false, dispatch.clone());
                                set_show_alert(e.to_string(), dispatch);
                            }
                        };
                    }
                    Err(e) => {
                        validation_errors.set(Rc::new(RefCell::new(e)));
                    }
                }
            });
        })
    };

    html! {
    <section class="bg-ct-blue-600 min-h-screen grid place-items-center">
      <div class="w-full">
        <h1 class="text-4xl xl:text-6xl text-center font-[600] text-ct-yellow-600 mb-4">
          {"Welcome Back"}
        </h1>
        <h2 class="text-lg text-center mb-4 text-ct-dark-200">
          {"Sign in to have access"}
        </h2>
          <form
            onsubmit={on_submit}
            class="max-w-md w-full mx-auto overflow-hidden shadow-lg bg-ct-dark-200 rounded-2xl p-8 space-y-5"
          >
            <FormInput label="username" name="username" input_type="username" input_ref={username_input_ref} handle_onchange={handle_username_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()} />
            <FormInput label="Password" name="password" input_type="password" input_ref={password_input_ref} handle_onchange={handle_password_input} errors={&*validation_errors} handle_on_input_blur={validate_input_on_blur.clone()}/>

            <div class="text-right">
              <a href="#">
                {"Forgot Password?"}
              </a>
            </div>
            <LoadingButton
              loading={store.page_loading}
              text_color={"text-ct-blue-600"}
            >
              {"Signin"}
            </LoadingButton>
            <span class="block">
              {"Need an account?"} {" "}
              <Link<Route> to={Route::SignUpPage} classes="text-ct-blue-600">{ "Sign Up Here" }</Link<Route>>
            </span>
          </form>
      </div>
    </section>
    }
}