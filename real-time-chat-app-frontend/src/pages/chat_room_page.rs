use crate::api::types::{ChatRoom, ChatRoomProps, UserWithRooms};
use crate::api::user_api::{create_chat_room, fetch_user_info, join_chat_room};
use crate::router::Route;
use gloo::timers::callback::Interval;
use js_sys::JsString;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, MessageEvent, WebSocket};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(ChatRoomPage)]
pub fn chat_room_page(ChatRoomProps { room_id, user_id }: &ChatRoomProps) -> Html {
    let ws = use_state(|| None::<WebSocket>);
    let messages = use_state(|| vec![]);
    let navigator = use_navigator().unwrap();
    let input_message = use_state(|| String::new());
    let user_info = use_state(|| None::<UserWithRooms>);
    let user_status = use_state(|| String::from("Loading...")); // Add status state
    let room_id = *room_id;
    let user_id = *user_id;

    // Connect WebSocket on mount
    // Fetch user information when component mounts
    {
        let user_info = user_info.clone();
        use_effect_with(user_id, move |&user_id| {
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
        });
    }

    {
        let user_status = user_status.clone();
        use_effect_with(user_id, move |&user_id| {
            let interval = Interval::new(1000, move || {
                // Poll every 1 seconds
                let user_status = user_status.clone();
                spawn_local(async move {
                    if let Ok(status_response) = fetch_user_info(user_id).await {
                        user_status.set(status_response.user.status.clone());
                    }
                });
            });

            || drop(interval) // Cleanup interval on component unmount
        });
    }

    {
        let ws = ws.clone();
        let messages = messages.clone();
        use_effect_with((room_id, user_id), move |&(room_id, user_id)| {
            let url = format!("ws://127.0.0.1:8000/ws/{}/{}", room_id, user_id);
            let websocket = WebSocket::new(&url).unwrap();

            let onmessage = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
                if let Ok(js_value) = e.data().dyn_into::<js_sys::JsString>() {
                    if let Some(msg) = js_value.as_string() {
                        println!("{:?}", *messages);
                        messages.set({
                            let mut updated_msgs = (*messages).clone(); // Clone the current state
                            updated_msgs.push(msg); // Add the new message
                            updated_msgs // Return the updated state
                        });
                    }
                }
            }) as Box<dyn FnMut(_)>);

            websocket.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            onmessage.forget();

            ws.set(Some(websocket));
            || ()
        });
    }

    // Handle input changes
    let on_input = {
        let input_message = input_message.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                input_message.set(input.value());
            }
        })
    };

    // Send a message
    let send_message = {
        let ws = ws.clone();
        let input_message = input_message.clone();
        Callback::from(move |_| {
            if let Some(ref websocket) = *ws {
                websocket.send_with_str(&(*input_message)).unwrap();
                input_message.set(String::new());
            }
        })
    };

    let username = user_info
        .as_ref()
        .and_then(|info| Some(info.user.username.clone()));
    html! {
        <div class="p-6 bg-gray-100 min-h-screen">
            <h1 class="text-2xl font-bold mb-4">{ format!("Chat Room: {}", room_id) }</h1>
            <h2 class="text-xl mb-4">
                { username.map_or_else(|| "Loading user...".to_string(), |name| format!("User: {}", name)) }
                <span class="text-sm text-gray-500">{ format!("({})", *user_status) }</span>
            </h2>
            <div class="border p-4 mb-4 h-64 overflow-y-scroll">
                { for messages.iter().map(|msg| html! { <div>{ msg }</div> }) }
            </div>
            <div class="flex gap-2">
                <input
                    type="text"
                    placeholder="Enter message"
                    value={(*input_message).clone()}
                    oninput={on_input}
                    class="px-4 py-2 border rounded-lg flex-grow"
                />
                <button onclick={send_message} class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600">
                    { "Send" }
                </button>
            </div>
        </div>
    }
}
