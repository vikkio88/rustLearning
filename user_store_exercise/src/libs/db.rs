use std::fs::{read_to_string, write};
use std::path::Path;

use crate::models::{Currency, Money, User};
const DB_FILE: &'static str = "./db.json";

fn generate_users() -> Vec<User> {
    vec![
        User::new("Mario Rossi".into(), Money::from_unit(1000, Currency::Euro)),
        User::new(
            "Eugenio Bianchi".into(),
            Money::from_unit(100, Currency::Euro),
        ),
    ]
}

pub struct Db {
    users: Vec<User>,
}

impl Db {
    pub fn new() -> Db {
        let mut db = Db { users: vec![] };
        db.load();
        db
    }

    pub fn load(&mut self) {
        let db_path = Path::new(DB_FILE);
        if db_path.exists() {
            let db_data = read_to_string(db_path).unwrap_or(String::new());
            self.users = serde_json::from_str(&db_data).unwrap_or(vec![]);
            return;
        }

        self.users = generate_users();
    }

    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|u| u.username == username)
    }

    pub fn update_user_by_id(&mut self, user: &User) -> bool {
        if let Some(user_to_update) = self.users.iter_mut().find(|u| u.id == user.id) {
            *user_to_update = user.clone();
            self.save();
            return true;
        }

        false
    }

    pub fn save(&self) {
        let data = serde_json::to_string(&self.users).unwrap();
        write(DB_FILE, data).expect("Unable to write file");
    }
}
