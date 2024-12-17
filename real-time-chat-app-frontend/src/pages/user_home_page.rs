use crate::api::types::UserWithRooms;
use crate::api::user_api::{api_signout_user, create_chat_room, fetch_user_info, join_chat_room};
use crate::router::Route;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserHomePageProps {
    pub user_id: i32,
}

#[function_component(UserHomePage)]
pub fn user_home_page(props: &UserHomePageProps) -> Html {
    let new_room_name = use_state(|| String::new());
    let access_code = use_state(|| String::new());
    let user_info = use_state(|| None::<UserWithRooms>);
    let navigator = use_navigator().unwrap();
    let user_id = props.user_id; // Use the passed user_id prop directly

    // Fetch user info when component mounts
    {
        let user_info = user_info.clone();
        use_effect_with(
            user_id,
            move |_| {
                let user_info = user_info.clone();
                spawn_local(async move {
                    match fetch_user_info(user_id).await {
                        Ok(data) => {
                            web_sys::console::log_1(&"Successfully fetched user info".into());
                            user_info.set(Some(data));
                        }
                        Err(err) => web_sys::console::log_1(
                            &format!("Failed to fetch user info: {}", err).into(),
                        ),
                    }
                });
                || ()
            }, // Dependency: only triggers if user_id changes
        );
    }

    // Update the input value for new room name
    let on_new_room_input = {
        let new_room_name = new_room_name.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                new_room_name.set(input.value());
            }
        })
    };

    // Update the input value for access code
    let on_access_code_input = {
        let access_code = access_code.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                access_code.set(input.value());
            }
        })
    };

    // Add a new room
    let add_room = {
        let new_room_name = new_room_name.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            if !new_room_name.is_empty() {
                let room_name = (*new_room_name).clone();
                let navigator = navigator.clone();
                spawn_local(async move {
                    match create_chat_room(room_name.clone(), user_id).await {
                        Ok(new_room) => {
                            web_sys::console::log_1(&"Successfully created chat room".into());
                            navigator.push(&Route::ChatRoomPage {
                                room_id: new_room.id,
                                user_id,
                            });
                        }
                        Err(err) => web_sys::console::log_1(
                            &format!("Failed to create room: {}", err).into(),
                        ),
                    }
                });
            }
        })
    };

    // Join a room
    let join_room = {
        let access_code = access_code.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            if !access_code.is_empty() {
                let room_id: Option<i32> = (*access_code).parse().ok();
                if let Some(room_id) = room_id {
                    let navigator = navigator.clone();
                    spawn_local(async move {
                        match join_chat_room(user_id, room_id).await {
                            Ok(_) => {
                                web_sys::console::log_1(&"Successfully joined room".into());
                                navigator.push(&Route::ChatRoomPage { room_id, user_id });
                            }
                            Err(err) => web_sys::console::log_1(
                                &format!("Failed to join room: {}", err).into(),
                            ),
                        }
                    });
                }
            }
        })
    };

    let sign_out = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            // Retrieve the user ID from local storage
            let user_id = {
                let window = web_sys::window().unwrap();
                let storage = window.local_storage().unwrap().unwrap();
                storage
                    .get_item("user_id")
                    .unwrap()
                    .and_then(|id| id.parse::<i32>().ok())
            };
            if let Some(user_id) = user_id {
                let navigator = navigator.clone(); // Clone navigator inside the closure
                spawn_local(async move {
                    match api_signout_user(user_id).await {
                        Ok(_) => {
                            // Redirect to the Home Page
                            navigator.push(&Route::HomePage);
                            web_sys::console::log_1(&"Successfully signed out".into());
                        }
                        Err(err) => {
                            web_sys::console::log_1(&format!("Sign out failed: {}", err).into());
                        }
                    }
                });
            } else {
                web_sys::console::log_1(&"User ID not found in local storage".into());
            }
        })
    };

    html! {
        <div class="p-6 bg-gray-100 min-h-screen">
            {
                if let Some(info) = &*user_info {
                    let user = &info.user;
                    html! {
                        <>
                            <div class="flex justify-between items-center mb-6">
                                <h1 class="text-2xl font-bold text-gray-800">
                                    { format!("Welcome, {}!", user.username) }
                                </h1>
                            </div>
                            <div class="flex items-center gap-4 mb-6">
                                <input
                                    type="text"
                                    placeholder="Enter chat room name"
                                    value={(*new_room_name).clone()}
                                    oninput={on_new_room_input}
                                    class="px-4 py-2 border rounded-lg w-64"
                                />
                                <button onclick={add_room.clone()} class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600">
                                    {"Create Room"}
                                </button>
                            </div>
                            <div class="flex items-center gap-4 mb-6">
                                <input
                                    type="text"
                                    placeholder="Enter room id"
                                    value={(*access_code).clone()}
                                    oninput={on_access_code_input}
                                    class="px-4 py-2 border rounded-lg w-64"
                                />
                                <button onclick={join_room.clone()} class="px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600">
                                    {"Join Room"}
                                </button>
                            </div>
                            <div class="flex justify-end mb-4">
                                <button onclick={sign_out.clone()} class="px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600">
                                    {"Sign Out"}
                                </button>
                            </div>
                        </>
                    }
                } else {
                    html! {
                        <div class="flex justify-center items-center min-h-screen">
                            <p class="text-xl font-medium text-gray-600">{"Loading user information..."}</p>
                        </div>
                    }
                }
            }
        </div>
    }
}
