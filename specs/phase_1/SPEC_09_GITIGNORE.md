# Spec 09: .gitignore - Git Ignore Configuration

**Fase**: 1 - Setup & Fundação  
**Componente**: Git Ignore Configuration (`.gitignore`)  
**Duração estimada**: 15 min  
**Dependência**: SPEC_01_CARGO_INIT

---

## Objetivo
Criar o arquivo `.gitignore` completo para ignorar arquivos gerados, builds, cache, IDEs e arquivos grandes (modelos ONNX).

---

## Código do .gitignore

```gitignore
# =============================================================================
# Rust Build Artifacts
# =============================================================================
/target/
**/*.rs.bk

# =============================================================================
# Cargo
# =============================================================================
Cargo.lock

# =============================================================================
# Models (Large Files - Downloaded at Runtime)
# =============================================================================
models/*.onnx
models/*.bin
models/*.onnx.data
models/*.onnx.json

# Model cache directories
.cache/
~/.cache/stem-separator/
~/Library/Caches/stem-separator/
%LOCALAPPDATA%\stem-separator\

# =============================================================================
# Build Outputs
# =============================================================================
*.out
*.bin
*.so
*.dll
*.dylib
*.lib
*.exp

# =============================================================================
# IDE / Editor
# =============================================================================
# VS Code
.vscode/
*.swp
*.swo
*~

# IntelliJ / CLion
.idea/
*.iml

# Vim
*.swp
*.swo
*~

# Emacs
*~
#*#
.#*

# VS Code (additional)
.vscode/*
!.vscode/settings.json
!.vscode/tasks.json
!.vscode/launch.json
!.vscode/extensions.json

# =============================================================================
# OS Specific
# =============================================================================
# macOS
.DS_Store
.AppleDouble
.LSOverride
Icon?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db

# Windows
Thumbs.db
ehthumbs.db
Desktop.ini
$RECYCLE.BIN/
*.lnk

# Linux
*~
.fuse_hidden*
.directory
.Trash-*

# =============================================================================
# Logs
# =============================================================================
*.log
logs/
*.log.*

# =============================================================================
# Testing / Coverage
# =============================================================================
coverage/
.coverage/
*.profraw
*.profdata
cobertura.xml
lcov.info

# =============================================================================
# Temporary Files
# =============================================================================
*.tmp
*.temp
tmp/
temp/
*.bak
*.swp
*.swo

# =============================================================================
# Environment / Secrets
# =============================================================================
.env
.env.local
.env.*.local
*.pem
*.key
*.crt
*.cert

# =============================================================================
# Documentation Build
# =============================================================================
doc/
target/doc/

# =============================================================================
# Profiling / Benchmarks
# =============================================================================
*.profraw
*.profdata
perf.data
perf.data.old

# =============================================================================
# Language Specific
# =============================================================================
# Python
__pycache__/
*.py[cod]
*.pyo
.pytest_cache/
.tox/
.venv/
venv/
env/

# Node.js
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# =============================================================================
# Docker
# =============================================================================
.dockerignore
Dockerfile*
docker-compose*.yml
.docker/

# =============================================================================
# Keep These (Do Not Ignore)
# =============================================================================
# !Cargo.lock          # Commit Cargo.lock for reproducible builds
# !.github/workflows/  # CI/CD workflows
# !.vscode/settings.json
# !.vscode/tasks.json
# !.vscode/launch.json
```

---

## Explicação das Seções

| Seção | Arquivos Ignorados | Por que |
|-------|-------------------|---------|
| Rust Build | `/target/` | Artefatos de build |
| Cargo | `Cargo.lock` | Commitado para reprodutibilidade |
| Models | `models/*.onnx` | Arquivos grandes (1.7GB), baixados em runtime |
| Cache | `.cache/`, `~/Library/Caches/` | Cache de modelos/downloads |
| Build Outputs | `*.so`, `*.dll`, `*.dylib` | Binários compilados |
| IDE | `.vscode/`, `.idea/`, `*.swp` | Configurações locais |
| OS | `.DS_Store`, `Thumbs.db` | Metadados do SO |
| Logs | `*.log` | Logs de execução |
| Temp | `*.tmp`, `tmp/` | Arquivos temporários |
| Secrets | `.env`, `*.pem`, `*.key` | Segredos não versionados |

---

## Arquivos que DEVEM ser Commitados

| Arquivo | Por que |
|---------|---------|
| `Cargo.lock` | Builds reprodutíveis |
| `.github/workflows/` | CI/CD pipelines |
| `.vscode/settings.json` | Configurações compartilhadas |
| `.vscode/tasks.json` | Tasks compartilhadas |
| `.vscode/launch.json` | Debug configs compartilhadas |

---

## Critérios de Aceite

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-09.1 | Arquivo existe | `ls -la .gitignore` |
| AC-09.2 | Ignora target/ | `git status` não mostra `target/` |
| AC-09.3 | Ignora modelos | `git status` não mostra `models/*.onnx` |
| AC-09.4 | Não ignora Cargo.lock | `git status` mostra `Cargo.lock` |
| AC-09.4 | Não ignora workflows | `git status` mostra `.github/workflows/` |

---

## Checklist

- [ ] `.gitignore` criado na raiz do projeto
- [ ] Seções organizadas por categoria
- [ ] Modelos ONNX ignorados
- [ ] Cache de modelos ignorado
- [ ] Arquivos de IDE ignorados
- [ ] Arquivos de SO ignorados
- [ ] `Cargo.lock` NÃO ignorado
- [ ] `.github/workflows/` NÃO ignorado
- [ ] `git status` limpo (apenas arquivos fonte)

---

## Verificação

```bash
# Verificar o que está sendo ignorado
git status --ignored

# Verificar se modelos são ignorados
touch models/test.onnx && git status --ignored | grep test.onnx

# Verificar se Cargo.lock NÃO é ignorado
git status | grep Cargo.lock
```