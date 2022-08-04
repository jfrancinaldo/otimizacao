# Comando Rust

## Instalando o rust

Acesse `rustup.rs` e copie o comando no terminal, quando rodar, aperte ENTER e espere a instalação acabar.

Adicione o seguinte conteúdo no arquivo `"\~/.bashrc"`:

```sh
. ~/.cargo/env
```

## Como compilar

Agora reinicie o terminal, e rode

```rust
cd pasta
cargo build --release
```

Copie o arquivo `target/release/frila` para aonde quiser, e pode rodar ele.

Como testar:

```sh
cargo run -- 05,59,46,43,33,56,11,39,25,21,12,23,47,35,53,31,09,48,03,02,30,28,38,36,40,27,34,24,42,22,58,20,32,17,51,55
```
