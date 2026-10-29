# Spec: GitHub Repository Setup

**Fase**: 1 - Setup & Fundação  
**Componente**: GitHub Repository Configuration  
**Duração estimada**: 15 min  

---

## Objetivo
Configurar o repositório GitHub com .gitignore apropriado e preparar para push inicial.

---

## Arquivos a Criar/Modificar

### 1. `.gitignore` (raiz do projeto)

```gitignore
# Rust
/target/
**/*.rs.bk
Cargo.lock

# IDE
.idea/
.vscode/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Sensíveis
.env
*.pem
*.key
*.crt
*.p12
*.pfx
secrets/
credentials/
*.env.local
*.env.*.local

# Build artifacts
*.wasm
dist/
build/

# Logs
*.log
logs/

# Test coverage
coverage/
*.profraw
*.profdata

# SPEC docs (internos)
SPEC*/
specs/
SPEC_*.md

# Binários locais
Rustem
rustem
*.exe

# Temporários
tmp/
temp/
*.tmp
*.temp
```

### 2. `.github/workflows/ci.yml` (opcional - CI básico)

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Check formatting
        run: cargo fmt -- --check
      - name: Check clippy
        run: cargo clippy -- -D warnings
      - name: Build
        run: cargo build --release
      - name: Run tests
        run: cargo test

  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Check formatting
        run: cargo fmt -- --check

  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Check clippy
        run: cargo clippy -- -D warnings
```

### 3. `README.md` (básico)

```markdown
# Rustem - Audio Stem Separator

High-quality audio stem separation using HTDemucs models via ONNX Runtime.

## Features
- Separate vocals, drums, bass, piano, and other instruments
- Support for WAV, FLAC, MP3 input formats
- ONNX Runtime inference (CPU/CUDA/CoreML)
- CLI interface with multiple commands

## Installation

```bash
cargo install --path .
```

## Usage

```bash
rustem separate input.wav --output ./output
rustem models list
rustem download htdemucs-v4
```

## Requirements
- Rust 1.70+
- ONNX Runtime (included via cargo feature)

## License
MIT OR Apache-2.0
```

---

## Critérios de Aceite

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-GH.1 | .gitignore ignora target/ | `git status` não mostra target/ |
| AC-GH.2 | .gitignore ignora Cargo.lock | `git status` não mostra Cargo.lock |
| AC-GH.3 | .gitignore ignora .env | `git status` não mostra .env |
| AC-GH.4 | .gitignore ignora SPEC* | `git status` não mostra pasta SPEC* |
| AC-GH.5 | .gitignore ignora segredos | `git status` não mostra .pem, .key, .env |
| AC-GH.6 | README.md existe | Arquivo presente na raiz |
| AC-GH.7 | CI workflow opcional | `.github/workflows/ci.yml` presente |

---

## Checklist

- [ ] Criar `.gitignore` na raiz
- [ ] Criar `README.md` na raiz
- [ ] Criar `.github/workflows/ci.yml` (opcional)
- [ ] Verificar `git status` limpo
- [ ] `git add . && git commit -m "chore: initial commit"`
- [ ] `git remote add origin <url>`
- [ ] `git push -u origin main`

---

## Notas

- O `.gitignore` deve ser o **primeiro** arquivo commitado
- `Cargo.lock` deve ser versionado para libs, mas para binários pode ser ignorado (aqui ignorado por ser binário)
- Pasta `SPEC*` ignorada por conter documentação interna de desenvolvimento
- Segredos nunca devem ser commitados - usar variáveis de ambiente no CI