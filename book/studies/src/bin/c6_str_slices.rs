fn concat_strings() {
    let publisher = "Code house".to_string();
    let authors = String::from("Ze and Will");
    let book: String = "Rust lang".into();

    let mut sentence = String::from("The book is ");
    sentence += &book;
    sentence += " from ";
    sentence.push_str(&publisher);
    sentence += ", written by ";
    sentence += &authors;

    println!("{}", sentence);
}

fn capacity_str() {
    let a = String::with_capacity(255);
    let b = String::from("ABC");

    println!("a: {} -> {}", a.capacity(), a);
    println!("a -> len: {}", a.len());
    println!("b: {} -> {}", b.capacity(), b);

    let mut game = String::from("wolfenstein");
    game.reserve(20);
    println!("game: {}", game.capacity());
    println!("game: {}", game.len());

    game.shrink_to_fit();
    println!("capacity com shrink: {}", game.capacity());
    println!("len com shrink: {}", game.len());
}

fn remove_by_pop() {
    let mut text = String::from("Rust");
    for _x in 0..text.len() {
        let re_turn = text.pop();
        match re_turn {
            Some(char) => println!("POP -> {}", char),
            None => println!("Without caracteres..."),
        }
    }
}

fn char_bytes_iter() {
    let item = String::from("rust");
    for (idx, c) in item.chars().enumerate() {
        println!("Char: {} -> {}", idx, c)
    }

    for b in item.bytes() {
        println!("Byte: {}", b)
    }
}

fn str_to_i8() {
    let a = String::from("42");
    let res = match a.parse::<i8>() {
        Ok(c) => c + 1,
        Err(_) => 0,
    };

    println!("{} + 1 = {}", a, res);
}

fn find_str() {
    let mut name = String::from("Luke Skywalker");
    let mut space = name.find(" ").unwrap_or(0);
    // println!("Space position is {}", space);
    let first_name: String = name.drain(..space).collect();

    space = name.find(" ").unwrap_or(0);
    let last_name: String = name.drain(space..).collect();

    println!("First name: {}", first_name);
    println!("Last name: {}", last_name);
    println!("Empty?: {}", name.is_empty());
}

fn main() {
    // concat_strings();
    // capacity_str();
    // remove_by_pop();
    // char_bytes_iter();
    // str_to_i8();
    find_str();
}
