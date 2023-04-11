use app::App;
use libs::{
    console::ctc,
    db::Db,
};

mod app;
mod libs;
mod models;

fn main() {
    let db = Db::new();
    let mut app = App::new(db);
    app.run();
}
