use yew::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

#[function_component(ChatRoomPage)]
pub fn chat_room_page() -> Html {
    // 模拟当前的 room_id
    let room_id = "1".to_string(); // 你可以动态替换此值
    let input_ref = use_node_ref(); // 获取输入框引用
    let messages = use_state(Vec::new); // 消息列表

    // 发送消息逻辑
    let send_message = {
        let input_ref = input_ref.clone();
        let messages = messages.clone();
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                let msg = input.value();
                if !msg.trim().is_empty() {
                    // 更新消息列表
                    messages.set({
                        let mut new_msgs = (*messages).clone();
                        new_msgs.push(format!("User 1: {}", msg));
                        new_msgs
                    });
                    input.set_value(""); // 清空输入框
                }
            }
        })
    };

    // 渲染 UI
    html! {
        <>
            // 显示 room_id
            <header class="bg-blue-600 text-white py-4 text-center">
                <h1 class="text-2xl font-bold">{ format!("Chat Room - Room {}", room_id) }</h1>
            </header>

            // 聊天框
            <section class="bg-gray-100 min-h-screen flex flex-col items-center justify-center">
                <div class="w-1/2 bg-white shadow-lg rounded-lg p-4">
                    <div class="h-64 overflow-y-auto border-b mb-4">
                        {
                            // 遍历消息并显示
                            for (*messages).iter().map(|msg| html! {
                                <div class="p-2">
                                    <span class="bg-gray-300 text-black rounded-lg px-3 py-1 inline-block">{msg}</span>
                                </div>
                            })
                        }
                    </div>
                    // 输入框和发送按钮
                    <div class="flex">
                        <input ref={input_ref} type="text" placeholder="Type your message here..."
                            class="flex-1 p-2 border rounded-l focus:outline-none" />
                        <button onclick={send_message} class="bg-blue-600 text-white px-4 py-2 rounded-r hover:bg-blue-500">
                            {"Send"}
                        </button>
                    </div>
                </div>
            </section>
        </>
    }
}
