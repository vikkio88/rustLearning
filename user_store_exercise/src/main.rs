use std::sync::{Arc, Mutex};

use app::App;
use libs::db::Db;

mod app;
mod libs;
mod models;

fn main() {
    let db = Db::new();
    let app = Arc::new(Mutex::new(App::new(db)));
    let t = Arc::clone(&app);

    ctrlc::set_handler(move || {
        println!("\n\nreceived SIGTERM\nclosing app.\n");
        
        // deadlock happens HERE as the app is running in the main loop and
        // I cannot call it.
        // to fix it I should send a message here maybe, but then I will need to
        // pass the receiver in any subloop?

        t.lock().unwrap().cleanup();
        println!("app shutting down. bye!\n\n");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    app.lock().unwrap().run();
}
