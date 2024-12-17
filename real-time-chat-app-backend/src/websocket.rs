use crate::presence::{PresenceActor, UserActivity, UserLogin, UserLogout};
use actix::prelude::*;
use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::collections::HashMap;
use std::collections::HashSet;

// 消息定义
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub room_id: i32,
    pub message: String,
    pub user_id: i32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub room_id: i32,
    pub addr: Recipient<ClientMessage>,
}

// 服务器管理聊天室内所有连接
pub struct ChatServer {
    rooms: HashMap<i32, HashSet<Recipient<ClientMessage>>>,
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            rooms: HashMap::new(),
        }
    }

    fn join_room(&mut self, room_id: i32, addr: Recipient<ClientMessage>) {
        self.rooms
            .entry(room_id)
            .or_insert_with(HashSet::new)
            .insert(addr);
    }

    fn broadcast(&self, room_id: i32, msg: ClientMessage) {
        if let Some(users) = self.rooms.get(&room_id) {
            for user in users {
                let _ = user.do_send(msg.clone());
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) {
        self.broadcast(msg.room_id, msg);
    }
}

impl Handler<JoinRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: JoinRoom, _: &mut Self::Context) {
        // 调用 ChatServer 的 join_room 方法，将客户端加入房间
        self.join_room(msg.room_id, msg.addr);
        // 打印日志，提示用户加入房间成功
        println!("User joined room {} successfully!", msg.room_id);
    }
}

// WebSocket 会话管理
pub struct ChatSession {
    user_id: i32,
    room_id: i32,
    server: Addr<ChatServer>,
    presence: Addr<PresenceActor>,
}

impl ChatSession {
    pub fn new(
        user_id: i32,
        room_id: i32,
        server: Addr<ChatServer>,
        presence: Addr<PresenceActor>,
    ) -> Self {
        Self {
            user_id,
            room_id,
            server,
            presence,
        }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address().recipient();

        // Announce user presence as 'online' when joining the room
        self.presence.do_send(UserLogin {
            user_id: self.user_id,
        });

        // Add user to the chat room
        self.server.do_send(JoinRoom {
            room_id: self.room_id,
            addr,
        });

        // Broadcast the join message
        self.server.do_send(ClientMessage {
            room_id: self.room_id,
            message: format!(
                "User {} joined room {} and is online",
                self.user_id, self.room_id
            ),
            user_id: self.user_id,
        });

        println!("User {} connected to room {}", self.user_id, self.room_id);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::prelude::Running {
        // 通知 PresenceActor 用户登出
        self.presence.do_send(UserLogout {
            user_id: self.user_id,
        });
        actix::prelude::Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            self.presence.do_send(UserActivity {
                user_id: self.user_id,
            });

            self.server.do_send(ClientMessage {
                room_id: self.room_id,
                message: text.to_string(),
                user_id: self.user_id,
            });

            //tx.text(format!("You: {}", text));
        }
    }
}

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    path: web::Path<(i32, i32)>,
    srv: web::Data<Addr<ChatServer>>,
    presence: web::Data<Addr<PresenceActor>>,
) -> Result<HttpResponse, Error> {
    let (room_id, user_id) = path.into_inner();
    ws::start(
        ChatSession::new(
            user_id,
            room_id,
            srv.get_ref().clone(),
            presence.get_ref().clone(),
        ),
        &req,
        stream,
    )
}

impl Handler<ClientMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        ctx.text(format!("User {}: {}", msg.user_id, msg.message));
    }
}
