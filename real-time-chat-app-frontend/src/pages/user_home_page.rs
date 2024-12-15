use yew::prelude::*;
use yew::Callback;

#[function_component(UserHomePage)]
pub fn user_home_page() -> Html {
    // Chat room state
    let chat_rooms = use_state(|| vec![
        String::from("General Chat")
    ]);

    // State for the new room name (for room creation)
    let new_room_name = use_state(|| String::new());

    // State for the access code (for joining a room)
    let access_code = use_state(|| String::new());

    // Add a new room
    let add_room = {
        let chat_rooms = chat_rooms.clone();
        let new_room_name = new_room_name.clone();
        Callback::from(move |_| {
            if !new_room_name.is_empty() {
                let mut rooms = (*chat_rooms).clone();
                rooms.push((*new_room_name).clone());
                chat_rooms.set(rooms);
                new_room_name.set(String::new()); // Clear the input after adding
            }
        })
    };

    // Join a room using access code
    let join_room = {
        let chat_rooms = chat_rooms.clone();
        let access_code = access_code.clone();
        Callback::from(move |_| {
            if !access_code.is_empty() {
                // Replace the logic below with an API call to validate the access code
                let room_name = format!("Room with Access Code: {}", *access_code);

                let mut rooms = (*chat_rooms).clone();
                if !rooms.contains(&room_name) {
                    rooms.push(room_name);
                    chat_rooms.set(rooms);
                }
                access_code.set(String::new()); // Clear the input after joining
            }
        })
    };

    // Update the input value as the user types (for room creation)
    let on_new_room_input = {
        let new_room_name = new_room_name.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                new_room_name.set(input.value());
            }
        })
    };

    // Update the input value as the user types (for access code)
    let on_access_code_input = {
        let access_code = access_code.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                access_code.set(input.value());
            }
        })
    };

    html! {
        <div class="p-6 bg-gray-100 min-h-screen">
            // Header
            <div class="flex justify-between items-center mb-6">
                <h1 class="text-2xl font-bold text-gray-800">{"Welcome to ChatApp!"}</h1>
            </div>

            // Create Chat Room Form
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

            // Join Chat Room Form
            <div class="flex items-center gap-4 mb-6">
                <input
                    type="text"
                    placeholder="Enter access code"
                    value={(*access_code).clone()}
                    oninput={on_access_code_input}
                    class="px-4 py-2 border rounded-lg w-64"
                />
                <button onclick={join_room.clone()} class="px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600">
                    {"Join Room"}
                </button>
            </div>

            // Chat Rooms List
            <div class="bg-white rounded-lg shadow-md p-4">
                <h2 class="text-xl font-semibold text-gray-800 mb-4">{"Your Chat Rooms"}</h2>
                <ul>
                    { for (*chat_rooms).iter().enumerate().map(|(index, room)| html! {
                        <li class="flex justify-between items-center mb-2 bg-gray-100 p-2 rounded-lg">
                            <span class="text-gray-700">{ room }</span>
                            <button class="px-3 py-1 text-sm bg-red-500 text-white rounded-lg hover:bg-red-600">
                                {"Delete"}
                            </button>
                        </li>
                    }) }
                </ul>
            </div>
        </div>
    }
}