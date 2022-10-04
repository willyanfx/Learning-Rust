#[derive(Debug)]
struct Fibonacci {
    curr: u32,
    next: u32
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        // Fibonacci is infinite, however we will condition to max 21
        
        if self.curr > 21 {
            None
        } else {
            Some(self.curr)
        }
    }
}

fn main() {
    let fib1 = Fibonacci {curr: 1, next: 1};
    println!("the first 4 number sequence is: ");
    for i in fib1.take(4) {
        println!("> {}", i);
    }
}

#[test]
fn name() {
    assert_eq!(false, 'a' == 'a');
}
