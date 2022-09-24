#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn cartesian() {
    let point_int = Point { x: 10, y: 12 };
    let point_flt = Point { x: 1.4, y: 12.3 };
    println!("{:?}", point_int);
    println!("{:?}", point_flt);
}

fn generic_sum<T: num::Num>(x: T, y: T) -> T {
    y + x
}

fn main() {
    // cartesian();
    println!("{}", generic_sum::<i32>(1, 2));
    println!("{}", generic_sum::<i16>(3, 4));
    println!("{}", generic_sum::<i8>(5, 6));
    println!("{}", generic_sum::<f32>(1.3, 2.24));
}
