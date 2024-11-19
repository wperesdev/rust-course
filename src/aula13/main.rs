fn main() {
    let faixa = 20..30;
    let animais = vec!["Coelho", "Gato", "Macaco"];
    
    for i in 1..10 {
        println!("O numero esta variando entre {}", i);
    }

    println!("\n------------------\n");

    for i in faixa {
        println!("O numero esta variando entre {}", i);
    }

    println!("\n------------------\n");

    for a in animais {
        println!("Os animais sao {}", a);
    }
}