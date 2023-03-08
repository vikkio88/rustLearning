#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn get_values(values: &mut Vec<i32>) -> (i32, i32) {
    let b = values.pop().unwrap();
    let a = values.pop().unwrap();
    (a, b)
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.len() < 1 {
        return None;
    }
    let mut values: Vec<i32> = Vec::new();

    for input in inputs {
        match input {
            CalculatorInput::Add => {
                if values.len() > 1 {
                    let (a, b) = get_values(&mut values);
                    values.push(a + b);
                } else {
                    return None;
                }
            }
            CalculatorInput::Subtract => {
                if values.len() > 1 {
                    let (a, b) = get_values(&mut values);
                    values.push(a - b);
                } else {
                    return None;
                }
            }
            CalculatorInput::Multiply => {
                if values.len() > 1 {
                    let (a, b) = get_values(&mut values);
                    values.push(a * b);
                } else {
                    return None;
                }
            }
            CalculatorInput::Divide => {
                if values.len() > 1 {
                    let (a, b) = get_values(&mut values);
                    values.push(a / b);
                } else {
                    return None;
                }
            }
            CalculatorInput::Value(v) => {
                values.push(*v);
            }
        };
    }
    if values.len() < 1 || values.len() > 1 {
        return None;
    }

    Some(values.pop()?)
}
