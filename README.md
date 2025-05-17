# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição

Este projeto implementa um sistema de busca eficiente em Rust, usando tabelas hash para indexação por nome, marca e categoria. É uma solução escalável, segura e de alto desempenho para lojas com grandes catálogos.

## Tecnologias

- Rust
- HashMap (std::collections)
- cargo test

## Executar o projeto

```bash
cargo run
```

## Executar os testes

```bash
cargo test
```

## Estrutura de dados

Usei múltiplos `HashMap<String, Vec<Product>>` para indexar os produtos por diferentes critérios de busca.

## Contribuição

Pull Requests são bem-vindos.

## Licença

MIT
