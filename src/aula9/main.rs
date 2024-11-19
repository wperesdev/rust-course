use std::io;

fn main () {
    let mut a = 15;
    let mut b = 60;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    println!("O maior divisor comum entre 15 e 60 eh: {}", a);
}