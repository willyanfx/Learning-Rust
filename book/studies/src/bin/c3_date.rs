use chrono::prelude::*;

fn print_today() {
    let t = Local::now();
    let (year, month, day) = (t.year(), t.month(), t.day());
    println!("Today is {}-{}-{}", year, month, day);
}
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn run_fn(function: fn()) {
    function()
}

fn main() {
    let func = print_today;
    run_fn(func);

    let x = 3;
    let y = 5;
    println!("{} + {} = {}", x, y, sum(x, y))
}
