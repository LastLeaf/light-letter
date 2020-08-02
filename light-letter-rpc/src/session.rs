use sha2::{Sha256, Digest};
use rand::Rng;

const SESSION_TIMEOUT: u32 = 86400;

lazy_static! {
    static ref RANDOM_SALT: &'static str = {
        let buf: [u8; 32] = rand::thread_rng().gen();
        Box::leak(format!("{:x?}", buf).into_boxed_str())
    };
}

fn hash_pwd(unique_salt: &str, expire_ts: u32, content: &str) -> String {
    let mut s = Sha256::new();
    s.input(format!("{}{}{}{}", unique_salt, expire_ts, content, *RANDOM_SALT).as_bytes());
    format!("{:x}", s.result())
}

fn current_ts() -> u32 {
    use std::time::*;
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as u32
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct SessionSig {
    unique_salt: String,
    expire_ts: u32,
    sig: String,
    content_json: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct Session {
    need_update: bool,
    pub login_user: Option<LoginUser>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct LoginUser {
    pub id: String,
    pub name: String,
}

impl Session {
    pub fn parse_sig_str(s: &str) -> Option<Session> {
        let state: SessionSig = serde_json::from_str(s).ok()?;
        if state.expire_ts <= current_ts() {
            return None
        }
        if state.sig != hash_pwd(&state.unique_salt, state.expire_ts, &state.content_json) {
            return None
        }
        let content = serde_json::from_str(&state.content_json).ok()?;
        Some(content)
    }

    pub fn generate_sig_str(&self) -> String {
        let buf: [u8; 8] = rand::thread_rng().gen();
        let unique_salt = format!("{:x?}", buf);
        let expire_ts = current_ts() + SESSION_TIMEOUT;
        let content_json = serde_json::to_string(self).unwrap();
        let sig = hash_pwd(&unique_salt, expire_ts, &content_json);
        serde_json::to_string(&SessionSig {
            unique_salt,
            expire_ts,
            sig,
            content_json,
        }).unwrap()
    }

    pub(crate) fn update(&mut self) {
        self.need_update = true;
    }

    pub(crate) fn need_update(&self) -> bool {
        self.need_update
    }
}
