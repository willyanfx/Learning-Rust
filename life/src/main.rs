extern crate rand;
use std::{thread, time};

fn main() {
    let mut world = [[0u8; 75]; 75];
    // let mut generations = 0;

    for i in 0..74 {
        for j in 0..74 {
            if rand::random() {
                world[i][j] = 1;
            } else {
                world[i][j] = 0;
            }
        }
    }

    println!("{:?}", world);
}
