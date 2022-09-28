macro_rules! hello {
    () => {
        println!("hello world macros");
    };
}

macro_rules! distance {
    ($a: ident, $b: expr, $c: expr) => {
        let $a = {
            if $b >= $c {
                $b - $c
            } else {
                $c - $b
            }
        };
    };
}

macro_rules! sum {
    ($a: ident, $b: expr, $($x: expr), *) => {
        let $a = {
            let mut temp = $b;
            $(
                temp = temp + $x;
            )*
            temp
        };
    }
}

macro_rules! write_html {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => (write!($w, "{}", $e).expect("no possible"));

    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        write!($w, "<{}>", stringify!($tag)).expect("No HTML");
        write_html!($w, $($inner)*);
        write!($w, "</{}>", stringify!($tag)).expect("NO HTML");
        write_html!($w, $($rest)*);
    }};
}

// TODO: learn how to write macros

fn main() {
    use std::fmt::Write;

    let mut out = String::new();

    write_html!(&mut out,
    html[
        head[title["Rust book"]]
        body[
            h1["Authors"]
                p["Will"]
                p["Ze"]
        ]
    ]);

    println!("{}", out);

    sum!(x, 1, 2, 3, 4, 5, 6, 7);
    sum!(y, 1.3, 2.4, 3.1, 4.98);
    println!("x => {}", x);
    println!("y => {}", y);
    distance!(x, 3, 5);
    distance!(y, 5, 3);
    println!("{}, {}", x, y);
    hello!();
}
