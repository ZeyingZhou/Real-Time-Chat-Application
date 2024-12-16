use actix::{Actor, AsyncContext, Context, Handler, Message};
use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

// Message for periodic presence checks
struct CheckPresence;

// Message for login event
pub struct UserLogin {
    pub user_id: i32,
}

// Message for activity (e.g., user sending a message)
pub struct UserActivity {
    pub user_id: i32,
}

// Message for logout event
pub struct UserLogout {
    pub user_id: i32,
}

// Implement Message trait for each event
impl Message for CheckPresence {
    type Result = ();
}

impl Message for UserLogin {
    type Result = ();
}

impl Message for UserActivity {
    type Result = ();
}

impl Message for UserLogout {
    type Result = ();
}

// PresenceActor manages user presence
pub struct PresenceActor {
    pool: Pool<SqliteConnectionManager>,
}

impl PresenceActor {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        PresenceActor { pool }
    }
}

impl Actor for PresenceActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Schedule periodic presence checks every 30 seconds
        ctx.run_interval(std::time::Duration::from_secs(30), |_act, ctx| {
            ctx.notify(CheckPresence);
        });
    }
}

//
// Handlers for various messages
//

// Periodic presence check handler
impl Handler<CheckPresence> for PresenceActor {
    type Result = ();

    fn handle(&mut self, _msg: CheckPresence, _: &mut Self::Context) {
        let conn = self.pool.get().unwrap();

        // Update users who are inactive
        if let Err(err) = conn.execute(
            "UPDATE users SET status = 'away' WHERE last_seen < DATETIME('now', '-2 minutes') AND status = 'online'",
            [],
        ) {
            eprintln!("Error updating user presence to 'away': {:?}", err);
        }
    }
}

// Handler for user login
impl Handler<UserLogin> for PresenceActor {
    type Result = ();

    fn handle(&mut self, msg: UserLogin, _: &mut Self::Context) {
        let conn = self.pool.get().unwrap();
        let now = Utc::now().to_rfc3339();

        if let Err(err) = conn.execute(
            "UPDATE users SET status = 'online', last_seen = ? WHERE id = ?",
            &[&now, &msg.user_id.to_string()],
        ) {
            eprintln!("Error updating user login time: {:?}", err);
        }
    }
}

// Handler for user activity (e.g., message sent)
impl Handler<UserActivity> for PresenceActor {
    type Result = ();

    fn handle(&mut self, msg: UserActivity, _: &mut Self::Context) {
        let conn = self.pool.get().unwrap();
        let now = Utc::now().to_rfc3339();

        // Update last_seen and reset status to 'online' if it was 'away'
        if let Err(err) = conn.execute(
            "UPDATE users SET status = 'online', last_seen = ? WHERE id = ? AND status != 'offline'",
            &[&now, &msg.user_id.to_string()],
        ) {
            eprintln!("Error updating user activity time: {:?}", err);
        }
    }
}

// Handler for user logout
impl Handler<UserLogout> for PresenceActor {
    type Result = ();

    fn handle(&mut self, msg: UserLogout, _: &mut Self::Context) {
        let conn = self.pool.get().unwrap();
        let now = Utc::now().to_rfc3339();

        if let Err(err) = conn.execute(
            "UPDATE users SET status = 'offline', last_seen = ? WHERE id = ?",
            &[&now, &msg.user_id.to_string()],
        ) {
            eprintln!("Error updating user logout time: {:?}", err);
        }
    }
}
