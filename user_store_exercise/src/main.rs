use libs::authenticate;
// use interactor::{pick_from_list, default_menu_cmd};
use models::{Money, User};

use crate::libs::console::{cls, etc, req_i, req_str};

mod libs;
mod models;

enum AppState {
    Login,
    Dashboard(User),
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
    let mut state = AppState::Login;

    loop {
        state = match state {
            AppState::Login => login(&users),
            AppState::Dashboard(user) => dashboard(user),
            AppState::Quit => {
                println!("Bye!\n");
                break;
            }
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
    let mut user = user;

    let mut choice = -1;
    while choice != 0 {
        cls();
        println!("Dashboard\n\n\tWelcome {}", user.username);
        println!("menu:\n1. View Funds\n2. Withdraw\n3. Deposit\n\t0. Logout");
        choice = req_i("[0-3]");

        match choice {
            1 => view_funds(&user),
            2 => withdraw(&mut user),
            3 => deposit(&mut user),
            _ => (),
        };
    }

    match choice {
        0 => {
            cls();
            AppState::Login
        }
        _ => AppState::Dashboard(user),
    }
}

fn view_funds(user: &User) {
    cls();
    println!("Balance: {}", user.balance);
    etc();
}
fn withdraw(user: &mut User) {
    let amount = Money::from_unit(10, user.balance.currency.clone());
    user.withdraw(amount.clone());
    cls();
    println!("Withdrew {}", amount);
    etc();
}
fn deposit(user: &mut User) {}
