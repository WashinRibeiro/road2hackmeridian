# Como Criar e Publicar sua Primeira Biblioteca em Rust

## O que é Rust?
*📌 Rust: Segurança, performace e produtividade*

**Definição:** Rust é uma linguagem de programação de sistemas focada em segurança de memória, concorrência e desempenho.

Porque Rust?
- **Zero-cost abstractions**: Performance próxima de C/C++.
- **Ownership**: Evita os erros mais comuns de memória
- **Ecossistema**: Usada em projetos como Firefox, Solana, Polkadot e Stellar


## Compilador + Ambiente
*⚡ Ferramentas para começar: rustup, cargo, rustc*
- **rustup**: Gerenciador de versões do Rust.
- **rustc**: Compilador oficial do Rust.
- **cargo**: Gerenciador de pacotes e ferramenta de build.


## Compilando e Executando:

```rust
// Compilar com rustc
rustc hello.rs

// Executar o binário
./hello
```

- Criamos um arquivo **hello.rs** com println! que é um macro para impressão no console.
- Usamos **rustc** para compilar diretamente, gerando um executável.
- Executamos o arquivo com **./hello** (ou **.\hello.exe** no windows)

## Tipos, Funções e Módulos
🛠️ Criando uma biblioteca com cargo para operações com u32

##### Criando o projeto
```rust
cargo new --lib calculator
```

##### Tipos
- u8, u32, u64 (inteiros sem sinal)
- i8, i32, i64 (inteiros com sinal)
- String (texto mutável)
- &str (fatia de texto imutável)
- Vec<T> (vetor, lista de elementos, ex: Vec<u8>)

##### Assinatura de funções
- fn: Palavra-chave para definir função
- ->: Indica o tipo de retorno da função
- return;: Retorno explícito (opcional na última expressão)

##### Módulos
Módulos servem para agrupar funções com um propósito comum. Podem estar dentro do mesmo arquivo ou em arquivos separados

```rust
// módulo dentro do mesmo arquivo
mod saudacoes {
    pub fn ola() {
        println!("Olá!");
    }
}

fn main() {
    saudacoes::ola();
}
```

##### Testes automatizados
No arquivo "./calculator/src/lib.rs" escrevemos os testes da biblioteca **calculator**

###### #[cfg(test)] 
- é um atributo de compilação condicional.
- Todo código dentro dele só é compilado e executado durante cargo test.
- Quando você roda cargo build ou cargo run, esse código é ignorado.
- É usado principalmente para testes e helpers de teste.
- Benefícios:
  - Reduz o tamanho do binário final.
  - Evita que código de teste vá para produção.
  - Permite separar código de teste do código de produção.

###### #[test]
- marca uma função como um teste.
- Funções com #[test] só são executadas quando você roda cargo test.
- Geralmente essas funções verificam se o código está funcionando corretamente, usando macros como assert!, assert_eq! ou assert_ne!.
- Não é necessário chamar a função manualmente; o próprio framework de teste do Rust a executa.

###### assert!
- Verifica se uma condição é verdadeira.
- Falha no teste se a condição for false.
```rust
assert!(2 + 2 == 4); // Passa
assert!(2 + 2 == 5); // Falha
```

###### assert_eq!
- Verifica se dois valores são iguais.
- - Mostra os valores esperados e obtidos se o teste falhar.
```rust
assert_eq!(2 + 2, 4); // Passa
assert_eq!(2 + 2, 5); // Falha mostrando "expected 5, got 4"
```

###### assert_ne!
- Verifica se dois valores são diferentes.
- Falha se os valores forem iguais.
```rust
assert_ne!(2 + 2, 5); // Passa
assert_ne!(2 + 2, 4); // Falha
```

###### Exemplo
```rust
#[cfg(test)]        // Só existe durante testes
mod tests {
    use super::*;

    #[test]        // Esta função é um teste
    fn soma_deve_funcionar() {
        assert_eq!(2 + 2, 4); // Verifica se 2 + 2 é igual a 4
    }
}
```
***#[cfg(test)]** → Inclui o módulo tests apenas durante testes.
**#[test]** → Diz que soma_deve_funcionar é um teste automático que o Rust deve executar.*


###### Rodar testes
```rust
cargo test
```


## Crates.io - Criando conta e publicando
*📌 Publicando a biblioteca calculadora no crates.io*

- Acesse crates.io e crie uma conta
- Gere um token de API em Account Settings > API Tokens
- Autenticar localmente
```rust
cargo login <seu-token>
```

- Verificar antes de publicar
```rust
cargo package
```

- Publicar no crates.io
```rust
cargo publish
```

## Baixando e usando Bibliotecas
Integrando a biblioteca publicada no Crates.io

- Criando um novo projeto
```rust
cargo new interactive_calculator
```

```toml
# Cargo.tml
[dependencies]
calculator-washin = "0.1.0"
```

## Executando um programa
```rust
cargo run
```