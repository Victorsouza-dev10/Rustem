# Spec 01: Cargo Init - Inicialização do Projeto

**Fase**: 1 - Setup & Fundação  
**Componente**: Cargo Project Initialization  
**Duração estimada**: 30 min

---

## Objetivo
Inicializar o projeto Rust com `cargo init` configurado corretamente.

---

## Comando

```bash
cd E:\Rustem
cargo init --name Rustem --edition 2024
```

---

## Resultado Esperado

```
Rustem/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs
│   └── lib.rs
├── .gitignore (básico)
└── README.md (básico)
```

---

## Cargo.toml Base (Mínimo)

```toml
[package]
name = "Rustem"
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"
description = "Separador de audio usando modelos HTDemucs ONNX"
repository = "https://github.com/Victorsouza-dev10/Rustem.git"
authors = ["Victorsouza-dev10 <victorsouza-dev10@users.noreply.github.com>"]
keywords = ["Rustem", "audio", "stem-separation", "demucs", "onnx", "dsp"]
categories = ["audio", "command-line-utilities", "machine-learning"]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1

[profile.bench]
debug = true
```

---

## Critérios de Aceite

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-01.1 | Projeto criado | `ls Cargo.toml src/main.rs src/lib.rs` |
| AC-01.2 | Nome correto | `grep 'name = "Rustem"' Cargo.toml` |
| AC-01.3 | Edition 2024 | `grep 'edition = "2024"' Cargo.toml` |
| AC-01.5 | Compila | `cargo check` sem erros |

---

## Checklist

- [ ] `cargo init --name Rustem --edition 2024`
- [ ] Verificar estrutura criada
- [ ] `cargo check` passa
- [ ] Commit inicial: `git init && git add . && git commit -m "chore: init Rustem projeto"`

---

## Notas

- Não adicionar dependências ainda (próxima spec)
- Não criar módulos ainda (próxima spec)
- Apenas inicialização mínima do Cargo