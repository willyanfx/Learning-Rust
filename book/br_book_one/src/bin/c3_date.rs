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

fn chars() {
    let dig: char = '\u{2764}';
    println!("{} é um dígito? {}", dig, dig.is_digit(10));

    let a: char = '→';
    let repr: String = a.escape_unicode().collect();
    println!("→ {}", repr);
    println!("Is alphabetic: {}", 'a'.is_alphabetic());
}
fn arrays() {
    let arr = ["livro", "de", "Rust"];
    println!("Programando em {}, acompanhando o {}", arr[2], arr[0]);

    let a_full = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let b_slice = &a_full[..];
    let c_slice = &a_full[3..5];
    println!(
        "[] has {}, {} and {} ",
        a_full.len(),
        b_slice.len(),
        c_slice.len()
    );

    for i in a_full {
        println!("for loop nº{}", i);
    }
}

fn tuple() {
    let a = ("Ru", 5, 't');
    // let a0 = a.0;
    // let a1 = a.1;
    // let a2 = a.2;
    let (a0, a1, a2) = a;
    println!("a0:{}, a1:{}, a2:{}", a0, a1, a2);
}

#[derive(Debug)]
enum Race {
    Dwarf,
    Elf,
    Ent,
    Hobbit,
    Men,
}

struct Creature {
    name: &'static str,
    gender: Race,
}

fn enums() {
    let elrond = Creature {
        name: "Elrond",
        gender: Race::Elf,
    };
    let treebeard = Creature {
        name: "Treebeard",
        gender: Race::Ent,
    };

    let arr = [elrond, treebeard];
    for creature in arr {
        println!("{} is {:?}", creature.name, creature.gender)
    }
}

mod compute {
    // private
    fn is_zero(number: i32) -> bool {
        if number == 0 {
            return true;
        };

        false
    }

    // public
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn check_grade(grade: f32) {
    match grade {
        0.0..=4.8 => println!("Não aprovado"),
        4.9..=5.9 => println!("De exame"),
        6.0..=10.0 => println!("Aprovado"),
        _ => println!("Nota inválida!"),
    }
}

fn check_tuple(t: (i32, i32)) {
    match t {
        (_x, 0) => println!("O segundo é zero"),
        (0, _x) => println!("O primeiro é zero"),
        _ => println!("Nenhum zero"),
    }
}

fn is_vowel_or_consonant(c: char) -> char {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => 'v',
        _ => 'c',
    }
}

fn vinculacao_de_match() {
    let name: &'static str = "Marcelo";
    let mut vowel_count = 0;
    let mut consonant_count = 0;

    for a in name.to_lowercase().chars() {
        match is_vowel_or_consonant(a) {
            'v' => vowel_count += 1,
            'c' => consonant_count += 1,
            _ => (),
        }
    }

    for a in name.to_lowercase().chars() {
        match is_vowel_or_consonant(a) {
            r @ 'v' => println!("{}", r),
            r @ 'c' => println!("{}", r),
            _ => (),
        }
    }

    println!(
        "The name {} has {} vowels e {} consonant",
        name, vowel_count, consonant_count
    );
}

fn main() {
    let func = print_today;
    run_fn(func);

    let x = 3;
    let y = 5;
    println!("{} + {} = {}", x, y, sum(x, y));

    chars();

    arrays();

    tuple();

    enums();

    use compute::add as my_add;
    println!("2 + 3 = {}", my_add(2, 3));
    check_grade(0.0);
    check_grade(3.2);
    check_grade(5.1);
    check_grade(8.3);

    check_tuple((0, 10));
    check_tuple((33, 0));
    check_tuple((8, 12));

    vinculacao_de_match()
}
