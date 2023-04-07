use libs::authenticate;
// use interactor::{pick_from_list, default_menu_cmd};
use models::{Money, User};

use crate::libs::console::{req_i, req_str};

mod libs;
mod models;

enum Actions {
    Withdraw,
    Deposit,
    Balance,
    Quit,
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

    let username = req_str("username:");
    let password = req_str("password:");

    let login_result = authenticate(username, password, &users);

    if login_result.is_none() {
        println!("Login Failed");
        return;
    }

    println!("logged in:\n\t{}", login_result.unwrap());
}
