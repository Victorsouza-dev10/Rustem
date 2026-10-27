# Spec 10: CI/CD - GitHub Actions Pipeline

**Fase**: 1 - Setup & Fundação  
**Componente**: CI/CD Pipeline (`.github/workflows/ci.yml`)  
**Duração estimada**: 45 min  
**Dependência**: SPEC_02_CARGO_TOML

---

## Objetivo
Configurar pipeline de CI/CD com GitHub Actions para: formatação, linting, build, testes, benchmarks e documentação.

---

## Arquivo: `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: "-D warnings"

jobs:
  # =========================================================================
  # Check: Format, Clippy, Build
  # =========================================================================
  check:
    name: Check (fmt, clippy, build)
    runs-on: ubuntu-latest
    timeout-minutes: 15
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Cache cargo
        uses: Swatinem/rust-cache@v2
        with:
          key: "${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}"

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Check clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Build
        run: cargo build --all-features

  # =========================================================================
  # Test: Unit and Integration Tests
  # =========================================================================
  test:
    name: Tests
    runs-on: ubuntu-latest
    timeout-minutes: 30
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo
        uses: Swatinem/rust-cache@v2
        with:
          key: "${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}"

      - name: Run unit tests
        run: cargo test --all-features --lib

      - name: Run integration tests
        run: cargo test --all-features --test integration_tests

      - name: Run doc tests
        run: cargo test --all-features --doc

  # =========================================================================
  # Bench: Performance Benchmarks
  # =========================================================================
  bench:
    name: Benchmarks
    runs-on: ubuntu-latest
    timeout-minutes: 30
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo
        uses: Swatinem/rust-cache@v2
        with:
          key: "${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}"

      - name: Run benchmarks (dry-run)
        run: cargo bench --all-features -- --no-run

  # =========================================================================
  # Docs: Documentation Generation
  # =========================================================================
  docs:
    name: Documentation
    runs-on: ubuntu-latest
    timeout-minutes: 10
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo
        uses: Swatinem/rust-cache@v2
        with:
          key: "${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}"

      - name: Generate docs
        run: cargo doc --all-features --no-deps --document-private-items

      - name: Check for broken links
        run: |
          # Check for broken intra-doc links
          cargo doc --all-features --no-deps 2>&1 | grep -i "warning.*unresolved" && exit 1 || true

  # =========================================================================
  # Security: Audit Dependencies
  # =========================================================================
  audit:
    name: Security Audit
    runs-on: ubuntu-latest
    timeout-minutes: 10
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install cargo-audit
        uses: actions-rs/cargo@v1
        with:
          command: install
          args: cargo-audit

      - name: Run security audit
        run: cargo audit --deny warnings

  # =========================================================================
  # Release: Build Release Artifacts (on tags)
  # =========================================================================
  release:
    name: Release
    runs-on: ubuntu-latest
    timeout-minutes: 30
    if: startsWith(github.ref, 'refs/tags/v')
    needs: [check, test]
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Rust toolchain
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo
        uses: Swatinem/rust-cache@v2

      - name: Build release
        run: cargo build --release --all-features

      - name: Strip binary
        run: strip target/release/stem-separator

      - name: Create release archive
        run: |
          mkdir -p release
          cp target/release/stem-separator release/
          cd release
          tar -czf stem-separator-${{ github.ref_name }}-${{ runner.os }}-${{ runner.arch }}.tar.gz stem-separator

      - name: Upload release asset
        uses: softprops/action-gh-release@v1
        with:
          files: release/*.tar.gz
          draft: false
          prerelease: false
```

---

## Explicação dos Jobs

| Job | Trigger | Descrição |
|-----|---------|-----------|
| `check` | push, PR | fmt, clippy, build |
| `test` | push, PR | Unit, integration, doc tests |
| `bench` | push to main | Benchmarks (dry-run) |
| `docs` | push, PR | Gerar docs, checar links |
| `audit` | push, PR | Auditoria de segurança (cargo-audit) |
| `release` | Tag `v*` | Build release, upload assets |

---

## Explicação dos Steps Principais

| Step | Ação |
|-------|------|
| `actions/checkout@v4` | Checkout do repo |
| `dtolnay/rust-toolchain@stable` | Instala Rust stable + componentes |
| `Swatinem/rust-cache@v2` | Cache de `~/.cargo` e `target/` |
| `cargo fmt --all -- --check` | Verifica formatação |
| `cargo clippy --all-targets --all-features -D warnings` | Linting estrito |
| `cargo build --all-features` | Build com todas features |
| `cargo test --all-features` | Testes com todas features |
| `cargo bench --no-run` | Compila benchmarks (não roda) |
| `cargo doc --no-deps` | Gera docs sem dependências |

---

## Cache Strategy

```yaml
- name: Cache cargo
  uses: Swatinem/rust-cache@v2
  with:
    key: "${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}"
```

- Cache key inclui OS + hash do Cargo.lock
- Cache compartilhado entre jobs
- Invalida automaticamente quando Cargo.lock muda

---

## Critérios de Aceite

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-10.1 | Pipeline roda em PR | PR abre → CI roda |
| AC-10.2 | Formatação verificada | PR com formatação errada falha |
| AC-10.3 | Clippy estrito | Warning vira erro |
| AC-10.4 | Testes rodam | `cargo test` no CI passa |
| AC-10.5 | Build release | Tag `v0.1.0` cria release |
| AC-10.6 | Cache funciona | Segunda execução mais rápida |
| AC-10.7 | Audit roda | `cargo audit` no CI |

---

## Checklist

- [ ] `.github/workflows/ci.yml` criado
- [ ] Jobs: check, test, bench, docs, audit, release
- [ ] Cache configurado com hash do Cargo.lock
- [ ] `cargo fmt --check` no job check
- [ ] `cargo clippy -D warnings` no job check
- [ ] `cargo test --all-features` no job test
- [ ] `cargo bench --no-run` no job bench
- [ ] `cargo doc --no-deps` no job docs
- [ ] `cargo audit` no job audit
- [ ] Release job só em tags `v*`
- [ ] Push para main → CI roda
- [ ] PR aberto → CI roda

---

## Verificação

```bash
# Testar localmente (simula CI)
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --all-features
cargo test --all-features
cargo bench --all-features -- --no-run
cargo doc --all-features --no-deps
cargo audit
```