use calculator_washin::calc1::{add, sub};
use calculator_washin::calc2::{multiply, rate};
use std::io;

fn main() {
    // Lendo a operação do usuário
    println!("Escolha a operação (+, -, *, /): ");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Erro");
    let operation = operation.trim();

    // Lendo os números
    println!("Digite o primeiro número: ");
    let mut num_a_string = String::new();
    io::stdin().read_line(&mut num_a_string).expect("Erro");
    let num_a: u32 = num_a_string.trim().parse().expect("Número inválido");

    println!("Digite o segundo número: ");
    let mut num_b_string = String::new();
    io::stdin().read_line(&mut num_b_string).expect("Erro");
    let num_b: u32 = num_b_string.trim().parse().expect("Número inválido");

    // Executando o cálculo
    let resultado = match operation {
        "+" => add(num_a, num_b),
        "-" => sub(num_a, num_b),
        "*" => multiply(num_a, num_b),
        "/" => rate(num_a, num_b),
        _ => {
            println!("Operação inválida");
            return;
        }
    };

    // Exibindo o resultado
    println!("Resultado: {} {} {} = {}", num_a, operation, num_b, resultado);
}
