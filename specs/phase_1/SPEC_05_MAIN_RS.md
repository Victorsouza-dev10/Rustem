# Spec 05: main.rs - CLI Entry Point

**Fase**: 1 - Setup & Fundação  
**Componente**: CLI Entry Point (`src/main.rs`)  
**Duração estimada**: 20 min  
**Dependência**: SPEC_04_LIB_RS, SPEC_06_ERRORS_RS, SPEC_07_LOGGING_RS

---

## Objetivo
Criar o entry point do CLI que inicializa logging e delega para o módulo de comandos.

---

## Código do main.rs

```rust
//! Stem Separator - Audio Stem Separation CLI
//!
//! High-quality audio stem separation using HTDemucs models via ONNX Runtime.

use stem_separator::cli::commands;
use stem_separator::utils::logging::init_logging;
use anyhow::Result;

fn main() -> Result<()> {
    // Initialize structured logging first
    init_logging();
    
    // Delegate to CLI commands
    commands::run()
}
```

---

## Dependências

```rust
use stem_separator::cli::commands;
use stem_separator::utils::logging::init_logging;
use anyhow::Result;
```

---

## Fluxo de Execução

```
main()
    │
    ├─► init_logging()     // Inicializa tracing subscriber
    │
    └─► commands::run()    // Delega para CLI module
                              │
                              ├─► Parse args (clap)
                              ├─► Match command
                              ├─► Execute command
                              └─► Return Result
```

---

## Tratamento de Erros

- `main` retorna `anyhow::Result<()>`
- `anyhow` converte automaticamente `StemError` → `anyhow::Error`
- Erros são logados via `tracing::error!` antes de sair
- Exit code: `0` sucesso, `1` erro

---

## Critérios de Aceite

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-05.1 | Compila | `cargo check` |
| AC-05.2 | Roda sem args | `cargo run -- --help` mostra help |
| AC-05.3 | Logging inicia | Logs aparecem no stderr |
| AC-05.4 | Erros propagam | Erro retorna exit code 1 |

---

## Checklist

- [ ] `src/main.rs` criado com código acima
- [ ] `cargo check` passa
- [ ] `cargo run -- --help` mostra help
- [ ] Logging aparece no stderr
- [ ] Erro retorna exit code 1

---

## Notas

- **Mínimo possível** - apenas inicializa logging e delega
- Toda lógica de CLI em `cli/commands.rs`
- `anyhow::Result` para propagação ergonômica de erros
- `init_logging()` **deve** ser primeira linha do `main`