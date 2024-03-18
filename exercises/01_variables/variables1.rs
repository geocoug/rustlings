// variables1.rs
//
// Make me compile!
//
// Execute `rustlings hint variables1` or use the `hint` watch subcommand for a
// hint.

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn main() {
    let mut x = 5;
    print_number(x);
    x = 10;
    print_number(x);
    println!("x has the value {}", x);
}
