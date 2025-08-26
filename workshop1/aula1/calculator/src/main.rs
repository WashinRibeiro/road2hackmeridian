use calculator_washin::calc1::{add, sub};
use calculator_washin::calc2::{multiply, rate};

fn main() {
    println!("\n---- Testando a Biblioteca Calculadora ----");

    let calc = add(10, 30);
    println!("Resultado {}", calc);

    println!("---- Fim dos testes manuais ----")
}