use crate::models::User;

pub mod console;

pub fn authenticate(username: String, password: String, users: &Vec<User>) -> Option<User> {
    if let Some(user) = users.iter().find(|u| u.username == username) {
        match user.check_password(password) {
            true => Some(user.clone()),
            false => None,
        }
    } else {
        return None;
    }
}
