## CREATE 

```cargo new <NAME_PROJECT>```

## BUILD 

```cargo build```

- Por padrão gera em DEBUG, para mudar para release basta informar --release

## RUN

```cargo run```

- Por padrão ao rodar o comando ele ja faz o build

## FORMAT CODE 

```cargo fmt```

- Formata o codigo

## DEPENDENCIAS 

- cargo-watch: 
    - Install: cargo install cargo-watch 
    - Description: Fica verificando as alterações do projeto, e faz o reload automaticamente quando detecta uma alteração 
    - Run: cargo watch -x run 

## INFORMATIONS 

- Por padrão toda variavel em rust é CONST, para poder alterar o valor utilizar ```mut``` na declaração.

- RAII: 