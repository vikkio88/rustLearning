use std::io::{stdin, stdout, Write};

pub fn cls() {
    print!("{}[2J", 27 as char);
}

pub fn req_str(prompt: &str) -> String {
    loop {
        let mut input = String::new();
        prompt_ln(prompt);
        if stdin().read_line(&mut input).is_ok() {
            return input.trim().to_string();
        } else {
            println!("something went wrong, try again...");
            continue;
        }
    }
}

pub fn req_i(prompt: &str) -> i32 {
    loop {
        let to_parse = req_str(prompt);
        match to_parse.parse() {
            Ok(result) => {
                return result;
            }
            Err(_) => {
                println!("{to_parse} wasn't a number.");
                continue;
            }
        };
    }
}

fn prompt_ln(message: &str) {
    print!("{message} > ");
    let _ = stdout().flush();
}
