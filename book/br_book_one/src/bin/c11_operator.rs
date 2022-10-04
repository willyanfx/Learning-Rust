use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Info {
    name: &'static str,
    age: i32,
    rating: i32,
}

fn write_info(info: &Info) -> io::Result<()> {
    let mut file = File::create("best_friends.txt")?;
    // ealy return on error
    file.write_all(format!("name: {}\n", info.name).as_bytes())?;
    file.write_all(format!("idade: {}\n", info.age).as_bytes())?; 
    file.write_all(format!("avaliação: {}\n", info.rating).as_bytes())?;
    Ok(())
  }

fn main() {
    let friend01 = Info {
        name: "Will G",
        age: 35,
        rating: 10
    };
        Err(e) => println!("Something went wrong -> {}", e),
        Ok(()) => println!("All good"),
    };
}
