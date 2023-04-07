use std::env;

enum Unit {
    Celsius,
    Fahrenheit,
}

fn parse_args() -> (i32, Unit) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("I need 2 args at least: a number and C or F\n\n");
    }
    let value = match args[1].parse::<i32>() {
        Ok(parsed) => parsed,
        Err(_) => panic!("{}", format!("{} is not a number!", args[1])),
    };

    let unit = match args[2].as_str() {
        "C" | "c" => Unit::Celsius,
        "F" | "f" => Unit::Fahrenheit,
        _ => panic!("{}", format!("{} is not F or C!", args[2])),
    };

    (value, unit)
}

fn to_far(value: i32) {
    let result = (value as f32 * 1.8) - 32.0;
    println!("\n\t{:.2} C\n\t{:.2} F\n", value, result);
}
fn to_c(value: i32) {
    let result = (value as f32 + 32.0) / 1.8;
    println!("\n\t{:.2} F\n\t{:.2} C\n", value, result);
}

fn main() {
    let (value, unit) = parse_args();

    match unit {
        Unit::Celsius => to_far(value),
        Unit::Fahrenheit => to_c(value),
    };
}
