use crate::models::User;

use self::db::Db;

pub mod console;
pub mod db;

pub fn authenticate(username: String, password: String, db: &Db) -> Option<User> {
    if let Some(user) = db.get_user_by_username(username.as_str()) {
        match user.check_password(password) {
            true => Some(user.clone()),
            false => None,
        }
    } else {
        return None;
    }
}
