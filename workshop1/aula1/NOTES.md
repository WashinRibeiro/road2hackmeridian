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