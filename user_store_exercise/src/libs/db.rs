use crate::models::User;

pub struct Db {
    users: Vec<User>,
}

impl Db {
    pub fn new() -> Db {
        Db { users: vec![] }
    }

    pub fn load(&mut self, users: Vec<User>) {
        self.users = users;
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|u| u.username == username)
    }

    pub fn update_user_by_id(&mut self, user: &User) -> bool {
        if let Some(user_to_update) = self.users.iter_mut().find(|u| u.id == user.id) {
            *user_to_update = user.clone();
            return true;
        }

        false
    }
}
