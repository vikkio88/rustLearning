use libs::authenticate;
// use interactor::{pick_from_list, default_menu_cmd};
use models::{Money, User};

use crate::libs::console::{cls, req_i, req_str};

mod libs;
mod models;

enum AppState {
    Login,
    Dashboard(User),
}

fn main() {
    let users = vec![
        User::new(
            "Mario Rossi".into(),
            Money::from_unit(1000, models::Currency::Euro),
        ),
        User::new(
            "Eugenio Bianchi".into(),
            Money::from_unit(100, models::Currency::Euro),
        ),
    ];
    let mut state = AppState::Login;

    loop {
        state = match state {
            AppState::Login => login(&users),
            AppState::Dashboard(user) => dashboard(user),
        };
    }
}

fn login(users: &Vec<User>) -> AppState {
    println!("Login\n");
    let username = req_str("username:");
    let password = req_str("password:");

    let login_result = authenticate(username, password, &users);

    if login_result.is_none() {
        println!("Login Failed\n");
        return AppState::Login;
    }

    let user = login_result.unwrap();
    AppState::Dashboard(user)
}

fn dashboard(user: User) -> AppState {
    cls();
    println!("Dashboard\n\n\tWelcome {}", user.username);

    let mut choice = -1;
    while choice < 0 {
        println!("menu:\n\t0. Logout");
        choice = req_i("[0-1]");
    }

    match choice {
        0 => {
            cls();
            AppState::Login
        }
        _ => AppState::Dashboard(user),
    }
}
