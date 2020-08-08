#[cfg(not(target_arch = "wasm32"))]
use tokio::fs;
use std::path::{PathBuf};
use sha2::{Sha256, Digest};
use rand::Rng;

pub const SESSION_TIMEOUT: u32 = 86400;

#[cfg(not(target_arch = "wasm32"))]
lazy_static! {
    static ref TMP_DIR: PathBuf = {
        let mut dir = std::env::temp_dir();
        dir.push("light-letter");
        std::fs::create_dir_all(&dir).expect("Failed initializing tmp files");
        dir
    };
}

#[cfg(not(target_arch = "wasm32"))]
async fn save_session_file(id: &str, content: &str) {
    let mut p = TMP_DIR.clone();
    p.push(id);
    if let Err(e) = fs::write(p, content).await {
        error!("Failed save session file: {}", e);
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn load_session_file(id: &str) -> Option<String> {
    let mut p = TMP_DIR.clone();
    p.push(id);
    fs::read_to_string(p).await.ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn hash_pwd(unique_salt: &str, expire_ts: u32, content: &str) -> String {
    let mut s = Sha256::new();
    crate::sites_config::SECURE_RANDOM_STRING.with(|srs| {
        s.input(format!("{}{}{}{}", unique_salt, expire_ts, content, &srs).as_bytes());
    });
    format!("{:x}", s.result())
}

#[cfg(not(target_arch = "wasm32"))]
fn current_ts() -> u32 {
    use std::time::*;
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap_or_default();
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

#[cfg(not(target_arch = "wasm32"))]
impl Session {
    pub async fn parse_sig_str(s: &str) -> Option<Session> {
        let s = load_session_file(s).await?;
        let state: SessionSig = serde_json::from_str(&s).ok()?;
        if state.expire_ts <= current_ts() {
            return None
        }
        if state.sig != hash_pwd(&state.unique_salt, state.expire_ts, &state.content_json) {
            return None
        }
        let content = serde_json::from_str(&state.content_json).ok()?;
        Some(content)
    }

    pub async fn generate_sig_str(&self) -> String {
        let buf: [u8; 8] = rand::thread_rng().gen();
        let unique_salt = format!("{:x?}", buf);
        let expire_ts = current_ts() + SESSION_TIMEOUT;
        let content_json = serde_json::to_string(self).unwrap();
        let sig = hash_pwd(&unique_salt, expire_ts, &content_json);
        let c = serde_json::to_string(&SessionSig {
            unique_salt,
            expire_ts,
            sig,
            content_json,
        }).unwrap();
        let id = format!("{}", uuid::Uuid::new_v4());
        save_session_file(&id, &c).await;
        id
    }

    pub(crate) fn update(&mut self) {
        self.need_update = true;
    }

    pub(crate) fn need_update(&self) -> bool {
        self.need_update
    }
}
