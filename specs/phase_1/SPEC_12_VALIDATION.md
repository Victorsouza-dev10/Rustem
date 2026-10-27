# Spec 12: Validation - Critérios de Aceite Fase 1

**Fase**: 1 - Setup & Fundação  
**Componente**: Validation Checklist - Critérios de Aceite Globais  
**Duração estimada**: 30 min  
**Dependência**: SPEC_01 a SPEC_11

---

## Objetivo
Definir lista consolidada de todos os critérios de aceite da Fase 1 para validação final antes de prosseguir para Fase 2.

---

## Matriz de Critérios de Aceite

### Infraestrutura Base

| ID | Critério | Comando de Verificação | Status |
|----|----------|------------------------|--------|
| AC-BASE-01 | Cargo.toml existe | `test -f Cargo.toml` | ⬜ |
| AC-BASE-02 | Nome do crate correto | `grep 'name = "stem-separator"' Cargo.toml` | ⬜ |
| AC-BASE-03 | Edition 2021 | `grep 'edition = "2021"' Cargo.toml` | ⬜ |
| AC-BASE-04 | License definida | `grep 'license' Cargo.toml` | ⬜ |
| AC-BASE-05 | Build passa | `cargo check --all-features` | ⬜ |
| AC-BASE-06 | Testes rodam | `cargo test` | ⬜ |
| AC-BASE-07 | Clippy limpo | `cargo clippy --all-targets --all-features -- -D warnings` | ⬜ |
| AC-BASE-08 | Formatação correta | `cargo fmt --all -- --check` | ⬜ |
| AC-BASE-09 | Docs geradas | `cargo doc --all-features --no-deps` | ⬜ |

### Estrutura de Módulos

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-MOD-01 | 7 diretórios em `src/` | `find src -maxdepth 1 -type d | wc -l` = 8 (inclui src/) |
| AC-MOD-02 | 7 arquivos mod.rs | `find src -name "mod.rs" \| wc -l` = 7 |
| AC-MOD-03 | 21 arquivos .rs | `find src -name "*.rs" \| wc -l` = 21 |
| AC-MOD-04 | Módulos declarados | `cargo check` sem erros de módulo |

### API Pública (lib.rs)

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-API-01 | `lib.rs` compila | `cargo check` |
| AC-API-02 | Re-exports funcionam | `use stem_separator::SeparationConfig;` compila |
| AC-API-03 | Constantes acessíveis | `stem_separator::DEFAULT_SAMPLE_RATE` compila |
| AC-API-04 | Docs geradas | `cargo doc --no-deps` sem warnings |
| AC-API-05 | Version acessível | `stem_separator::VERSION` retorna versão |

### CLI Entry Point

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-CLI-01 | Compila | `cargo check` |
| AC-CLI-02 | Help funciona | `cargo run -- --help` mostra help |
| AC-CLI-03 | Logging inicia | Logs aparecem no stderr |
| AC-CLI-04 | Erros propagam | Erro retorna exit code 1 |

### Error Handling

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-ERR-01 | Compila | `cargo check` |
| AC-ERR-02 | `From` impls funcionam | `let err: StemError = io_err.into();` |
| AC-ERR-03 | `context()` funciona | `err.context("msg")` adiciona prefixo |
| AC-ERR-04 | `bail!` macro | `bail!("error")` retorna Err |
| AC-ERR-05 | Testes passam | `cargo test utils::errors` passa |

### Logging

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-LOG-01 | Compila | `cargo check` |
| AC-LOG-02 | `init_logging()` não panica | `init_logging()` roda sem panic |
| AC-LOG-03 | `RUST_LOG` funciona | `RUST_LOG=trace cargo run` mostra trace |
| AC-LOG-04 | Formato compacto | Logs em uma linha |
| AC-LOG-05 | JSON logging | `init_json_logging()` produz JSON lines |

### Convenções (CLAUDE.md)

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-CLD-01 | Arquivo existe | `test -f CLAUDE.md` |
| AC-CLD-02 | Conteúdo completo | `grep -c "##" CLAUDE.md` ≥ 10 seções |
| AC-CLD-03 | Convenções claras | Seções: Rust, Error Handling, Async, Logging, Tests, Docs, Versioning, Features, Code Style |

### Git Ignore

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-GIT-01 | Arquivo existe | `test -f .gitignore` |
| AC-GIT-02 | Ignora target/ | `git status` não mostra `target/` |
| AC-GIT-03 | Ignora modelos | `git status` não mostra `models/*.onnx` |
| AC-GIT-04 | Não ignora Cargo.lock | `git status` mostra `Cargo.lock` |
| AC-GIT-05 | Não ignora workflows | `git status` mostra `.github/workflows/` |

### CI/CD

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-CI-01 | Pipeline roda em PR | PR abre → CI roda |
| AC-CI-02 | Formatação verificada | PR com formatação errada falha |
| AC-CI-03 | Clippy estrito | Warning vira erro |
| AC-CI-04 | Testes rodam | `cargo test` no CI passa |
| AC-CI-05 | Build release | Tag `v0.1.0` cria release |
| AC-CI-06 | Cache funciona | Segunda execução mais rápida |
| AC-CI-07 | Audit roda | `cargo audit` no CI |

### Formatação (rustfmt.toml)

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-FMT-01 | Arquivo existe | `test -f .rustfmt.toml` |
| AC-FMT-02 | `cargo fmt --check` passa | `cargo fmt --all -- --check` |
| AC-FMT-03 | Largura 100 chars | Linhas não passam 100 chars |
| AC-FMT-04 | Imports organizados | `cargo fmt` reorganiza imports |

---

## Validação Completa (Script de Verificação)

```bash
#!/bin/bash
# validate_phase1.sh - Script de validação completa Fase 1

set -e

echo "=== VALIDAÇÃO FASE 1 ==="
echo ""

# Cores
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

check() {
    local name="$1"
    local cmd="$2"
    
    echo -n "Checking $name... "
    if eval "$cmd" >/dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
        return 0
    else
        echo -e "${RED}✗${NC}"
        return 1
    fi
}

failed=0

# Base
check "Cargo.toml exists" "test -f Cargo.toml" || failed=1
check "Cargo.toml name" "grep -q 'name = \"stem-separator\"' Cargo.toml" || failed=1
check "Edition 2021" "grep -q 'edition = \"2021\"' Cargo.toml" || failed=1

# Build
check "cargo check" "cargo check --all-features >/dev/null 2>&1" || failed=1
check "cargo test" "cargo test >/dev/null 2>&1" || failed=1
check "cargo clippy" "cargo clippy --all-targets --all-features -- -D warnings >/dev/null 2>&1" || failed=1
check "cargo fmt" "cargo fmt --all -- --check >/dev/null 2>&1" || failed=1
check "cargo doc" "cargo doc --all-features --no-deps >/dev/null 2>&1" || failed=1

# Module structure
check "7 modules dirs" "[ $(find src -maxdepth 1 -type d | wc -l) -eq 8 ]" || failed=1
check "7 mod.rs files" "[ $(find src -name 'mod.rs' | wc -l) -eq 7 ]" || failed=1
check "21 .rs files" "[ $(find src -name '*.rs' | wc -l) -eq 21 ]" || failed=1
check "cargo check modules" "cargo check >/dev/null 2>&1" || failed=1

# API
check "lib.rs compiles" "cargo check >/dev/null 2>&1" || failed=1
check "Public API accessible" "grep -q 'pub use separation' src/lib.rs" || failed=1

# Files exist
check "main.rs" "test -f src/main.rs" || failed=1
check "lib.rs" "test -f src/lib.rs" || failed=1
check "errors.rs" "test -f src/utils/errors.rs" || failed=1
check "logging.rs" "test -f src/utils/logging.rs" || failed=1
check "CLAUDE.md" "test -f CLAUDE.md" || failed=1
check ".gitignore" "test -f .gitignore" || failed=1
check ".rustfmt.toml" "test -f .rustfmt.toml" || failed=1
check "CI workflow" "test -f .github/workflows/ci.yml" || failed=1

# Git ignore
check "gitignore exists" "test -f .gitignore" || failed=1

echo ""
if [ $failed -eq 0 ]; then
    echo -e "\033[0;32m✓ ALL CHECKS PASSED\033[0m"
    exit 0
else
    echo -e "\033[0;31m✗ $failed CHECKS FAILED\033[0m"
    exit 1
fi
```

---

## Critérios de Aprovação da Fase 1

Para aprovar a Fase 1 e prosseguir para Fase 2, **TODOS** os critérios acima devem passar:

- [ ] **Todos** os AC-BASE passam
- [ ] **Todos** os AC-MOD passam
- [ ] **Todos** os AC-API passam
- [ ] **Todos** os AC-CLI passam
- [ ] **Todos** os AC-ERR passam
- [ ] **Todos** os AC-LOG passam
- [ ] **Todos** os AC-CLD passam
- [ ] **Todos** os AC-GIT passam
- [ ] **Todos** os AC-CI passam
- [ ] **Todos** os AC-FMT passam

---

## Comando de Validação Único

```bash
# Executar validação completa
bash validate_phase1.sh

# Ou manualmente:
cargo check --all-features && \
cargo test && \
cargo clippy --all-targets --all-features -- -D warnings && \
cargo fmt --all -- --check && \
cargo doc --all-features --no-deps
```

---

## Próximos Passos Após Aprovação

Após **TODOS** os critérios passarem:

1. ✅ Fase 1 aprovada
2. → Iniciar **Fase 2: Audio I/O & Preprocessing**
   - Audio I/O (WAV/FLAC/MP3)
   - Resampling (rubato)
   - STFT/ISTFT (daspect)
   - Normalização LUFS (EBU R128)

---

## Contato para Dúvidas

Em caso de dúvidas sobre algum critério, consultar:
- Espec correspondente (`SPEC_XX_*.md`)
- `CLAUDE.md` para convenções
- `cargo check` output para erros de compilação