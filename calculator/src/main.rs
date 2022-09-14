#[warn(dead_code)]
#[derive(Debug)]
enum Operators {
    Add,
    Subtract,
    Divide,
    Multiply,
}

struct Calc {
    first_number: i32,
    operator: Operators,
    second_number: i32,
    result: i32,
}

fn main() {
    let mut state = Calc {
        first_number: 5,
        operator: Operators::Add,
        second_number: 2,
        result: 0,
    };

    state.result = operate(state.operator, state.first_number, state.second_number);

    println!("Result, {:?}", state.result);
}

fn operate(operator: Operators, first: i32, second: i32) -> i32 {
    match operator {
        Operators::Add => first + second,
        Operators::Subtract => first - second,
        Operators::Multiply => first * second,
        Operators::Divide => first / second,
        _ => 0,
    }
}
