//sorry no comments here , i'm  too lazy to add ts rn
#![allow(clippy::needless_return)]

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use lazy_static::lazy_static;
use sha2::{Digest, Sha256};
use spin::Mutex;

#[derive(Clone)]
struct User{
    username: String,
    salt: [u8; 16],
    hash: [u8; 32]
}

struct Db{
    users: Vec<User>,
    current: Option<String>,
    counter: u64,
}

lazy_static! {
    static ref DB: Mutex<Db> = Mutex::new(Db{
        users: Vec::new(),
        current: None,
        counter: 0,
    });
}

fn hash_password(salt: &[u8; 16], password: &str) -> [u8; 32]{
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(password.as_bytes());
    let out = hasher.finalize();
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&out[..32]);
    arr
}

fn derive_salt(username: &str, counter: u64) -> [u8; 16]{
    let mut hasher = Sha256::new();
    hasher.update(username.as_bytes());
    hasher.update(counter.to_le_bytes());
    let out = hasher.finalize();
    let mut salt = [0u8; 16];
    salt.copy_from_slice(&out[..16]);
    salt
}

pub fn ensure_default_admin(){
    let mut db = DB.lock();
    if db.users.is_empty(){
        let salt = derive_salt("admin", 1);
        let hash = hash_password(&salt, "admin");
        db.users.push(User{
            username: "admin".to_string(),
            salt,
            hash,
        });
    }
}

pub fn user_exists(username: &str) -> bool {
    let db = DB.lock();
    db.users.iter().any(|user| user.username == username)
}

pub fn add_user(username: &str, password: &str) -> Result<(), &'static str> {
    if username.trim().is_empty() || password.trim().is_empty() {
        return Err("username/password cannot be empty");
    }
    let mut db = DB.lock();
    if db.users.iter().any(|user| user.username == username) {
        return Err("user already exists");
    }
    db.counter = db.counter.wrapping_add(1);
    if db.counter == 0 {db.counter = 1;}
    let salt = derive_salt(username, db.counter);
    let hash = hash_password(&salt, password);
    db.users.push(User{
        username: username.to_string(),
        salt,
        hash,
    });
    Ok(())
    }

    pub fn auth(username: &str, password: &str) -> bool {
        let mut db = DB.lock();
        if let Some(user) = db.users.iter().find(|user| user.username == username) {
            let hash = hash_password(&user.salt, password);
            if hash == user.hash {
                db.current = Some(username.to_string());
                return true;
            }
        }
        false
    }

    pub fn current_user() -> Option<String> { DB.lock().current.clone() }
    pub fn logout(){DB.lock().current = None;}