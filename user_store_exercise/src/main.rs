use libs::{
    authenticate,
    console::ctc,
    db::{self, Db},
};
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

fn cleanup() {
    println!("running cleanup...");
    println!("cleanup done.");
}

fn main() {
    ctc(cleanup);
    let mut db = db::Db::new();
    db.load(vec![
        User::new(
            "Mario Rossi".into(),
            Money::from_unit(1000, models::Currency::Euro),
        ),
        User::new(
            "Eugenio Bianchi".into(),
            Money::from_unit(100, models::Currency::Euro),
        ),
    ]);

    let mut state = AppState::Login;

    loop {
        state = match state {
            AppState::Login => login(&db),
            AppState::Dashboard(user) => {
                let newstate = dashboard(user, &mut db);
                // I want also to return user here
                newstate
            }
            AppState::Quit => {
                println!("Bye!\n");
                break;
            }
        };
    }
}

fn login(db: &Db) -> AppState {
    println!("Login\n");
    let username = req_str("username:");
    let password = req_str("password:");

    let login_result = authenticate(username, password, &db);

    if login_result.is_none() {
        println!("Login Failed\n");
        return AppState::Login;
    }

    let user = login_result.unwrap();
    AppState::Dashboard(user)
}

fn dashboard(user: User, db: &mut Db) -> AppState {
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
            0 => etc(),
            _ => {
                println!("{} is not a valid choice", choice);
                etc();
            }
        };
        db.update_user_by_id(&user);
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
    let unit = req_pos_amount(
        format!(
            "how much {} you want to withdraw?",
            user.balance.currency.clone()
        )
        .as_str(),
    );
    let amount = Money::from_unit(unit, user.balance.currency.clone());
    let am_str = amount.to_string();
    user.withdraw(amount);
    println!("\nWithdrew {}", am_str);
    etc();
}
fn deposit(user: &mut User) {
    let unit = req_pos_amount(
        format!(
            "how much {} you want to deposit?",
            user.balance.currency.clone()
        )
        .as_str(),
    );
    let amount = Money::from_unit(unit, user.balance.currency.clone());
    let am_str = amount.to_string();
    user.deposit(amount);

    println!("\nDeposited {}", am_str);
    etc();
}

fn req_pos_amount(prompt: &str) -> i32 {
    let mut unit = -1;
    while unit <= 0 {
        unit = req_i(prompt);

        if unit <= 0 {
            println!("\n\tError: it has to be an amount > 0");
        }
    }

    return unit;
}
