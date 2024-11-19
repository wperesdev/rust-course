fn alguma_fn(par_a: f32, par_b: f32) -> f32 {
    println!("Essa funcao devolve um valor flutuante");
    10 as f32 // Use `as` para fazer cast para float 32
}

fn main() {
    alguma_fn(10.0, 20.0);
}