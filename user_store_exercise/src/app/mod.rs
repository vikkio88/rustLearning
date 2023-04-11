use crate::libs::console::{cls, ctc, etc, req_i, req_str};
use crate::models::{Money, User};

use crate::libs::{authenticate, db::Db};

pub enum AppState {
    Login,
    Dashboard(User),
    Quit,
}

pub struct App {
    state: AppState,
    db: Db,
}

impl App {
    pub fn new(db: Db) -> App {
        App {
            state: AppState::Login,
            db,
        }
    }

    pub fn run(&mut self) {
        loop {
            let new_state = match &self.state {
                AppState::Login => self.login(),
                AppState::Dashboard(user) => {
                    let newstate = self.dashboard(user.clone());
                    // I want also to return user here
                    newstate
                }
                AppState::Quit => {
                    println!("Bye!\n");
                    break;
                }
            };
            self.state = new_state;
        }
    }
    pub fn cleanup(&self) {
        println!("running cleanup...");
        self.db.save();
        println!("cleanup done.");
    }

    fn login(&self) -> AppState {
        println!("Login\n");
        let username = req_str("username:");
        let password = req_str("password:");

        let login_result = authenticate(username, password, &self.db);

        if login_result.is_none() {
            println!("Login Failed\n");
            return AppState::Login;
        }

        let user = login_result.unwrap();
        AppState::Dashboard(user)
    }

    fn dashboard(&mut self, user: User) -> AppState {
        let mut user = user;

        let mut choice = -1;
        while choice != 0 {
            cls();
            println!("Dashboard\n\n\tWelcome {}", user.username);
            println!("menu:\n1. View Funds\n2. Withdraw\n3. Deposit\n4. Move Money\n5. Change Password\n\t0. Logout");
            choice = req_i("[0-3]");

            match choice {
                1 => self.view_funds(&user),
                2 => self.withdraw(&mut user),
                3 => self.deposit(&mut user),
                5 => self.change_password(&mut user),
                0 => etc(),
                _ => {
                    println!("{} is not a valid choice", choice);
                    etc();
                }
            };
            self.db.update_user_by_id(&user);
        }

        match choice {
            0 => {
                cls();
                AppState::Login
            }
            _ => AppState::Dashboard(user),
        }
    }

    fn view_funds(&self, user: &User) {
        cls();
        println!("Balance: {}", user.balance);
        etc();
    }

    fn withdraw(&self, user: &mut User) {
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

    fn deposit(&self, user: &mut User) {
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

    fn change_password(&self, user: &mut User) {
        let new_password = req_str("insert new password");
        user.change_password(new_password);
        println!("\nPassword Changed!");
        etc();
    }
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
