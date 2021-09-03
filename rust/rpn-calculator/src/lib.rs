#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    if inputs.len() < 3 && inputs.len() != 1 {
        return None;
    }

    let mut stack = Vec::<i32>::new();
    for input in inputs.into_iter() {
        match input {
            CalculatorInput::Add => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left + right);
            }
            CalculatorInput::Subtract => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left - right);
            }
            CalculatorInput::Multiply => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left * right);
            }
            CalculatorInput::Divide => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left.div_euclid(right));
            }
            CalculatorInput::Value(v) => stack.push(*v),
        }
    }

    stack.pop()
}
