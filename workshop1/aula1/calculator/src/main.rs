use calculator_washin::calc1::{add, sub};
use calculator_washin::calc2::{multiply, rate};
use calculator_washin::calc3::{pot, log};

fn main() {
    println!("\n---- Testando a Biblioteca Calculadora ----");

    let calc_add = add(10, 30);
    let calc_sub = sub(30, 5);
    let calc_mul = multiply(5, 4);
    let calc_rate = rate(20, 2);
    let calc_pot = pot(2.0, 3);
    let calc_log = log(3.0, 9.0);

    println!("Resultado add: {}", calc_add);
    println!("Resultado sub: {}", calc_sub);
    println!("Resultado mul: {}", calc_mul);
    println!("Resultado rate: {}", calc_rate);
    println!("Resultado pot: {}", calc_pot);
    println!("Resultado log: {}", calc_log);

    println!("---- Fim dos testes manuais ----")
}