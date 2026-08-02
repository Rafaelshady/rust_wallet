# Carteira de Investimentos Fullstack com Rust

Aplicação web criada durante o desafio da DIO para acompanhar compras de ativos e a evolução de uma carteira de investimentos.

O projeto usa Rust no back-end e na renderização das páginas. O Axum recebe as requisições, o SQLx acessa o PostgreSQL e o Askama transforma os dados em HTML.

## Funcionalidades

- cadastro automático do usuário no primeiro login;
- autenticação por cookie com token JWT;
- catálogo inicial com Bitcoin, Ethereum, Solana, Dólar e Real;
- registro de compras por ativo;
- histórico de compras;
- dashboard com valor investido, valor atual e resultado da carteira;
- comparação do valor atual em dólar e real;
- destaque visual para ganhos e perdas;
- validação de quantidade, preço e ativo informado.

## Como o cálculo funciona

Para cada compra:

```text
valor investido = preço de compra × quantidade
valor atual = preço atual do ativo × quantidade
resultado = valor atual - valor investido
```

O resumo do dashboard soma esses valores para todas as posições do usuário. O preço atual vem da coluna `unit_value` da tabela `assets`.

Os preços dos ativos são armazenados em dólar. A comparação em real usa:

```text
valor em reais = valor em dólares × cotação USD/BRL
```

A cotação didática pode ser alterada na variável `USD_BRL_RATE` do arquivo `.env`. Além da comparação exibida no resumo, Dólar e Real permanecem disponíveis no catálogo, sem substituir Ethereum e Solana.

## Tecnologias

- Rust
- Axum
- Askama
- SQLx
- PostgreSQL
- Docker Compose
- Tailwind CSS via CDN

## Pré-requisitos

- Rust e Cargo;
- Docker Desktop;
- SQLx CLI:

```powershell
cargo install sqlx-cli --no-default-features --features postgres
```

## Executando o projeto

Na pasta do projeto:

```powershell
docker compose up -d
sqlx migrate run
cargo run
```

Crie o arquivo `.env` a partir do `.env.example` e ajuste a cotação quando necessário:

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres
USD_BRL_RATE=5.50
```

Acesse [http://localhost:3000](http://localhost:3000). No primeiro login, o usuário é criado com a senha informada.

Durante o desenvolvimento, o PostgreSQL pode continuar no Docker. Ao alterar código Rust ou templates Askama, pare o servidor com `Ctrl+C` e execute `cargo run` novamente para recompilar.

## Validação

```powershell
cargo fmt --check
cargo check
cargo test
```

Os testes de repositório usam `#[sqlx::test]` e criam bancos temporários. Por isso, o PostgreSQL precisa estar em execução e o usuário configurado em `DATABASE_URL` precisa ter permissão para criar bancos.

## Estrutura principal

```text
src/
├── auth/          # autenticação do administrador e dos usuários
├── routes/        # rotas da API e das páginas
├── app.rs         # montagem da aplicação
├── error.rs       # erros HTTP da aplicação
├── models.rs      # modelos de domínio e resumo da carteira
└── repository.rs  # consultas ao PostgreSQL
templates/         # páginas Askama
migrations/        # criação e evolução do banco
```

## Rotas

- `GET /login`: página de login;
- `POST /login`: autentica ou cadastra o usuário;
- `GET /assets`: dashboard autenticado;
- `POST /assets`: registra uma compra;
- `GET /logout`: encerra a sessão;
- `/api/assets`: lista e administra o catálogo de ativos com autenticação de administrador.

## Próximas melhorias

- mover os segredos de administrador e JWT para variáveis de ambiente;
- criar uma área administrativa protegida para cadastrar e editar ativos;
- usar um tipo decimal para valores monetários em vez de `f64`;
- adicionar mensagens de sucesso e erro nos formulários;
- criar testes de integração para login e registro de compras.

## Observação de segurança

Este é um projeto educacional. Antes de publicar em produção, substitua os segredos fixos no código por variáveis de ambiente, use cookies seguros em HTTPS e revise a estratégia de cadastro automático.
