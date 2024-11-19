fn dobro(num: i32) -> i32 {
    2*num
}

fn maior(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
}

fn main() {
    println!("O maior numero entre 5 e 4 eh {}", maior(5, 4));
}