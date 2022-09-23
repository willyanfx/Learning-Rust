use std::cmp::Ordering;
use std::ops::Add;

trait User {
    fn new(username: &'static str) -> Self;

    fn username(&self) -> &'static str; // return the login defined on `new`

    fn login(&self) -> &'static str;
    fn logout(&self) -> &'static str;
    fn is_logged_in(&self) -> bool {
        false
    }
}

#[derive(Debug)]
struct Admin {
    username: &'static str,
}
struct Operator {
    username: &'static str,
}
struct BasicUser {
    username: &'static str,
}

impl User for Admin {
    fn new(username: &'static str) -> Self {
        Admin { username: username }
    }

    fn username(&self) -> &'static str {
        self.username
    }

    fn login(&self) -> &'static str {
        "Usuário do tipo ADMIN entrou no sistema"
    }

    fn logout(&self) -> &'static str {
        "Usuário do tipo ADMIN saiu do sistema"
    }
}

impl User for Operator {
    fn new(username: &'static str) -> Operator {
        Operator { username: username }
    }
    fn username(&self) -> &'static str {
        self.username
    }
    fn login(&self) -> &'static str {
        "Usuário do tipo OPERADOR entrou no sistema"
    }
    fn logout(&self) -> &'static str {
        "Usuário do tipo OPERADOR saiu do sistema"
    }
}

impl User for BasicUser {
    fn new(username: &'static str) -> BasicUser {
        BasicUser { username: username }
    }
    fn username(&self) -> &'static str {
        self.username
    }
    fn login(&self) -> &'static str {
        "Usuário do tipo BÁSICO entrou no sistema"
    }
    fn logout(&self) -> &'static str {
        "Usuário do tipo BÁSICO saiu do sistema"
    }
}

fn basic() {
    let admin: Admin = User::new("Teobaldo");
    println!("{:?}", admin);
    println!("Welcome user: {}", admin.username());
    println!("{}", admin.login());
    println!("{}", admin.logout());

    let operador: Operator = User::new("PessoaQualquer");
    println!("Welcome user {}", operador.username());
    println!("{}", operador.login());
    println!("{}", operador.logout());
    let basic_user: BasicUser = User::new("PessoaQualquer2");
    println!("Welcome user {}", basic_user.username());
    println!("{}", basic_user.login());
    println!("{}", basic_user.logout());
}

#[derive(PartialEq)]
struct MyStruct {
    member: i32,
}

enum BookFormat {
    Paperback,
    Ebook,
}

struct Book {
    isbn: i32,
    #[allow(dead_code)]
    title: &'static str,
    #[allow(dead_code)]
    format: BookFormat,
}

impl PartialEq for Book {
    fn eq(&self, other: &Book) -> bool {
        self.isbn == other.isbn
    }
}

fn partial_eq() {
    let b1 = Book {
        isbn: 1234567890,
        title: "O Senhor dos Anéis",
        format: BookFormat::Paperback,
    };
    let b2 = Book {
        isbn: 1234567890,
        title: "O Senhor dos Anéis",
        format: BookFormat::Paperback,
    };
    let b3 = Book {
        isbn: 1234567810,
        title: "O Hobbit",
        format: BookFormat::Ebook,
    };
    println!("{}", b1 == b2);
    println!("{}", b2 == b3);
    println!("{}", b1 == b3);
}

#[derive(PartialOrd, PartialEq, Eq, Clone, Copy)]
struct Person {
    age: i32,
    name: &'static str,
}

impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        (self.age).cmp(&(other.age))
    }
}

fn older(p1: Person, p2: Person) {
    if p1 > p2 {
        println!("{} tem mais anos de vida do que {}", p1.name, p2.name);
    } else {
        println!("{} tem mais anos de vida do que {}", p2.name, p1.name);
    }
}

fn partial_ord() {
    let p1 = Person {
        age: 6,
        name: "Nicolas",
    };
    let p2 = Person {
        age: 32,
        name: "Willian",
    };
    let p3 = Person {
        age: 40,
        name: "Marcelo",
    };

    older(p1, p2);
    older(p2, p3);
    older(p1, p3);
}
impl Add<i32> for Person {
    type Output = i32;

    fn add(self, b: i32) -> i32 {
        self.age + b
    }
}

fn add_age() {
    let p1 = Person {
        name: "Connor MacLeod",
        age: 52,
    };
    let x = p1 + 10;
    println!("A nova idade de {} é {}", p1.name, x);
}

fn main() {
    add_age();

    partial_ord();

    partial_eq();

    basic();

    let a = MyStruct { member: 1 };
    let b = MyStruct { member: 1 };
    println!("{}", a == b);
}
