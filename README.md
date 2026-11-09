# 🦀 Sistema de E-Commerce e Carrinho de Compras em Rust

> **Projeto Desenvolvido para o Treinamento / Formação Rust na Digital Innovation One (DIO)**

Uma aplicação Web Full-Stack completa desenvolvida em **Rust**, utilizando o framework **Rocket**, arquitetura **MVC (Model-View-Controller)**, renderização server-side com templates **Tera** e persistência de dados local com **SQLite (`rusqlite`)**.

---

## 🚀 Funcionalidades

- **Gestão Completa de Clientes (CRUD)**:
  - Listagem de clientes cadastrados.
  - Cadastro de novos clientes com validações.
  - Edição de dados existentes.
  - Exclusão com confirmação.
- **Catálogo de Produtos & Carrinho de Compras**:
  - Exibição de produtos disponíveis com fotos, descrições e preços.
  - Adição de itens dinamicamente ao carrinho.
  - Visualização em tempo real do carrinho de compras com cálculo automático de subtotal e total.
  - Remoção de itens do carrinho.
  - Fechamento e persistência de pedidos.
- **Interface Responsiva**:
  - Páginas renderizadas no backend com **Tera Templates** e estilizadas com **Bootstrap 5**.

---

## 🛠️ Tecnologias e Bibliotecas

| Tecnologia / Crate | Versão | Descrição |
| :--- | :--- | :--- |
| **Rust** | 2021 Edition | Linguagem de programação moderna, segura e de alta performance |
| **Rocket** | `0.5.0` | Framework web assíncrono para Rust |
| **rocket_dyn_templates (Tera)** | `0.1.0` | Motor de templates para renderização dinâmica de páginas HTML |
| **rusqlite** | `0.31.0` | Driver nativo para conexão e manipulação do banco de dados SQLite |
| **serde** / **serde_json** | `1.0` | Serialização e desserialização eficiente de estruturas de dados |
| **chrono** | `0.4` | Tratamento e formatação de datas e horários |
| **dotenv** | `0.15.0` | Carregamento de variáveis de ambiente via arquivo `.env` |
| **Bootstrap** | `5.x` | Framework CSS para design responsivo |

---

## 📁 Estrutura do Projeto

```text
treinamento_rust/
├── db/
│   ├── rusqlite.db       # Banco de dados SQLite pré-populado
│   └── script.sql        # Script DDL/DML para criação das tabelas e dados iniciais
├── src/
│   ├── config/           # Configuração de conexão com o banco de dados (cnn.rs)
│   ├── controllers/      # Controladores HTTP (home, clientes, compras)
│   ├── dtos/             # Data Transfer Objects (transferência de dados entre camadas)
│   ├── models/           # Estruturas de dados de domínio (Cliente, Produto, Pedido)
│   ├── modelviews/       # Estruturas formatadas para apresentação nas views
│   ├── repositorios/     # Camada de acesso a dados (SQL queries com rusqlite)
│   ├── servicos/         # Regras de negócio e lógica de aplicação
│   └── main.rs           # Ponto de entrada da aplicação e registro de rotas
├── static/               # Arquivos estáticos (CSS, JS, Bootstrap)
├── templates/            # Telas HTML dinâmicas com sintaxe Tera
│   ├── clientes/         # Views de listagem, cadastro e edição de clientes
│   ├── compras/          # Views de compras e carrinho
│   ├── home/             # Página inicial
│   ├── layouts/          # Template base (application.html.tera)
│   └── partials/         # Componentes reutilizáveis (header, footer, head)
├── Cargo.toml            # Manifesto de dependências do Rust
├── Rocket.toml           # Configurações do servidor web Rocket (portas, workers, etc.)
├── .env                  # Variáveis de ambiente da aplicação
└── historico_mudancas/   # Registro das aulas e evolução ao longo do treinamento
```

---

## 💻 Como Executar o Projeto

### Pré-requisitos
- Ter o **Rust** e o **Cargo** instalados. Caso não tenha, instale através do [rustup.rs](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Passos para execução

1. **Clone este repositório**:
   ```bash
   git clone https://github.com/pablohenriquesilva945-cmd/treinamento_rust.git
   cd treinamento_rust
   ```

2. **Execute a aplicação**:
   ```bash
   cargo run
   ```

3. **Acesse no navegador**:
   - Abra [http://127.0.0.1:8080](http://127.0.0.1:8080)
   - O servidor iniciará automaticamente com o banco SQLite embutido (`db/rusqlite.db`).

---

## 🌐 Rotas Principais

- `GET /` — Página inicial com atalho para a loja
- `GET /clientes` — Listagem de clientes
- `GET /clientes/novo` — Formulário de cadastro de cliente
- `POST /clientes` — Salvar novo cliente
- `GET /clientes/<id>/editar` — Editar cliente existente
- `POST /clientes/<id>/alterar` — Salvar alterações do cliente
- `GET /clientes/<id>/excluir` — Remover cliente
- `GET /compras` — Listagem de produtos para compra
- `POST /compras/adicionar` — Adicionar produto ao carrinho
- `GET /carrinho` — Exibir carrinho atual e resumo da compra
- `GET /compras/excluir/<id>` — Remover item do carrinho

---

## 🎓 Autor e Créditos

- **Aluno:** Pablo Henrique Silva
- **Curso:** Treinamento / Formação Rust
- **Plataforma:** [Digital Innovation One (DIO)](https://www.dio.me/)
- **Instrutor Original:** Danilo Aparecido dos Santos (Didox)
