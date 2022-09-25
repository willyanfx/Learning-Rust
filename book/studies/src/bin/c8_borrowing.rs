fn print_v(pv: &mut Vec<i32>) {
    pv[1] = 44;
    println!("v in print_v:  {:?}", pv);
}

fn borrowing_v() {
    let mut v = vec![1, 2, 3];
    print_v(&mut v);
    println!("v in borrowing_v:  {:?}", v);
}

fn ownership() {
    let mut v = vec![1, 2, 3];
    let mut v2 = v.clone();
    println!("v:  {:?}", v);
    println!("v2: {:?}", v2);
    v[1] = 5;
    v2[1] = 88;
    println!("v:  {:?}", v);
    println!("v2: {:?}", v2);
}

fn main() {
    ownership();
    borrowing_v();
}
