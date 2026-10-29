# Rustem - Status do Projeto & Resumo da Implementação

> **Última Atualização**: 2026-10-27  
> **Fase**: 1 - Setup & Fundação (Parcialmente Completa)  
> **Branch**: master  
> **Commit**: 5850154 - "chore: init Rustem projeto"

---

## Visão Geral

Rustem é um separador de stems de áudio usando modelos HTDemucs ONNX. O projeto é organizado em fases, com a Fase 1 (Setup & Fundação) atualmente em progresso.

---

## Fase 1: Setup & Fundação - Status da Implementação

### ✅ SPEC_01: Cargo Init - **COMPLETO**
- Projeto inicializado com `cargo init --name Rustem --edition 2024`
- Estrutura básica criada: `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/lib.rs`
- Commit inicial: `5850154`

### ✅ SPEC_02: Cargo.toml Dependências - **COMPLETO**
**Cargo.toml atual** inclui todas as dependências especificadas:
- **Audio I/O**: `hound` (3.5), `lewton` (0.10), `mp3lame-encoder` (0.2), `symphonia` (0.5)
- **DSP/Processamento de Sinal**: `ruststft` (0.4), `rubato` (0.15), `realfft` (3.5), `num-complex` (0.4), `ndarray` (0.16 + blas)
- **ML/ONNX**: `ort` (2.0.0-rc.15 com `load-dynamic`, `download-binaries`, `copy-dylibs`)
- **CLI**: `clap` (4.5 + derive/env), `anyhow` (1.0), `tracing` (0.1), `tracing-subscriber` (0.3 + env-filter)
- **Serialização**: `serde` (1.0 + derive), `serde_json` (1.0), `toml` (0.8)
- **Utilitários**: `rayon` (1.10), `tempfile` (3.8), `walkdir` (2.5), `indicatif` (0.17)
- **Features**: `default = []`, `cuda = ["ort/cuda"]`, `coreml = ["ort/coreml"]`
- **Perfis**: Release (opt-level=3, lto=true, codegen-units=1, strip=true), Bench (debug=true)

### ⚠️ SPEC_03: Estrutura de Módulos - **PARCIALMENTE COMPLETO**
**Diretórios criados** (7/7): `audio/`, `models/`, `separation/`, `metrics/`, `cli/`, `utils/`, raiz `src/`

**Arquivos mod.rs criados** (7/7): Todos os diretórios de módulos têm `mod.rs` com declarações `pub mod` corretas

**Arquivos .rs criados** (16/21 - **5 FALTANDO**):

| Módulo | Arquivos Especificados | Status |
|--------|----------------------|--------|
| `src/audio/` | mod.rs, io.rs, preprocessing.rs, postprocessing.rs | ✅ 4/4 |
| `src/models/` | mod.rs, onnx.rs, demucs.rs, ensemble.rs, registry.rs | ✅ 5/5 |
| `src/separation/` | mod.rs, **pipeline.rs**, stem.rs, config.rs | ❌ 3/4 (pipeline.rs faltando) |
| `src/metrics/` | mod.rs, **sdr.rs**, **sir.rs**, **sar.rs** | ❌ 1/4 (3 faltando) |
| `src/cli/` | mod.rs, **args.rs**, **commands.rs** | ❌ 1/3 (2 faltando) |
| `src/utils/` | mod.rs, **errors.rs**, **logging.rs** | ❌ 1/3 (2 faltando) |

### ❌ SPEC_04: lib.rs API Pública - **INCOMPLETO**
**Estado atual**: `lib.rs` mínimo com apenas constante `VERSION` e teste básico
**Esperado**: Re-exports completos de toda API pública (SeparationConfig, SeparationOutput, Stem, AudioBuffer, QualityMetrics, etc.), constantes (DEFAULT_SAMPLE_RATE, STFT_WINDOW_SIZE, NUM_STEMS, STEM_NAMES), e documentação

### ❌ SPEC_05: main.rs Entry Point CLI - **INCOMPLETO**
**Estado atual**: `println!("Hello, world!");`
**Esperado**: Inicializar logging e delegar para `commands::run()`

### ❌ SPEC_06: errors.rs - **FALTANDO**
Arquivo não existe. Spec requer:
- Enum `StemError` com 20+ variants usando `thiserror`
- Type alias `Result<T>`
- Trait `ResultExt` com `context()`/`with_context()`
- Macro `bail!`
- Testes unitários

### ❌ SPEC_07: logging.rs - **FALTANDO**
Arquivo não existe. Spec requer:
- `init_logging()` com suporte a `RUST_LOG`
- `init_logging_with_filter()`
- `init_json_logging()`
- Macro `span!`
- Testes unitários

### ❌ SPEC_08: CLAUDE.md - **FALTANDO**
Arquivo não existe. Spec requer documento de convenções do projeto

### ❌ SPEC_09: .gitignore - **FALTANDO**
Arquivo não existe. Spec requer gitignore abrangente com ignores de modelos/cache/IDE/SO

### ❌ SPEC_10: CI/CD (.github/workflows/ci.yml) - **FALTANDO**
Diretório e arquivo não existem. Spec requer 6 jobs: check, test, bench, docs, audit, release

### ❌ SPEC_11: rustfmt.toml - **FALTANDO**
Arquivo não existe. Spec requer config de formatação (max_width=100, tab_spaces=4, etc.)

### ❌ SPEC_12: Checklist de Validação - **NÃO EXECUTÁVEL**
Não dá pra rodar validação até todas as specs acima estarem completas

---

## Avaliação de Qualidade do Código

### ✅ Módulos Bem Implementados

| Módulo | Qualidade | Observações |
|--------|-----------|-------------|
| `audio/io.rs` | Alta | Leitura WAV/FLAC/MP3 completa, escrita WAV, AudioBuffer com conversões, testes |
| `audio/preprocessing.rs` | Alta | Resampling (rubato), normalização LUFS, STFT (daspect), testes |
| `audio/postprocessing.rs` | Alta | Crossfade, normalização loudness/peak, dithering, overlap-add, testes |
| `models/onnx.rs` | Alta | Gestão de sessão, helpers de inferência, conversão de tensores, testes |
| `models/demucs.rs` | Média | Wrapper existe mas tem erros de compilação (variáveis não definidas) |
| `models/ensemble.rs` | Alta | Todas as 5 estratégias implementadas com testes |
| `models/registry.rs` | Média | Carregamento/download assíncrono de modelos, trait objects, mas integração ensemble incompleta |
| `separation/stem.rs` | Alta | Enum Stem completo, StemOutputs, SeparationMetadata, StemMetrics, testes |
| `separation/config.rs` | Alta | SeparationConfig completo com builder pattern, validação, enums Device/AudioFormat, testes |

### ⚠️ Módulos com Problemas

| Módulo | Problema |
|--------|----------|
| `models/demucs.rs` | Referencia `preprocessed_stft` e `stem_samples` não definidos (linhas 71, 105, 125) |
| `models/registry.rs` | Usa `reqwest` e `tokio` que **não estão** no Cargo.toml |

### ❌ Módulos Faltando (Apenas Stubs)

| Módulo | Status |
|--------|--------|
| `separation/pipeline.rs` | Não existe |
| `metrics/sdr.rs` | Não existe |
| `metrics/sir.rs` | Não existe |
| `metrics/sar.rs` | Não existe |
| `cli/args.rs` | Não existe |
| `cli/commands.rs` | Não existe |
| `utils/errors.rs` | Não existe |
| `utils/logging.rs` | Não existe |

---

## Análise de Dependências

### Faltando no Cargo.toml (referenciados no código):
- `thiserror` - Necessário para `errors.rs` (SPEC_06)
- `reqwest` - Usado em `registry.rs` para download de modelos
- `tokio` - Usado em `registry.rs` para operações assíncronas

### Diferenças de Versão da Spec:
| Crate | Versão Spec | Versão Atual |
|-------|-------------|--------------|
| `lewton` | 0.11 | 0.10 |
| `mp3lame-encoder` | 0.4 | 0.2 |
| `daspect` | 0.6 | **Não está no Cargo.toml** (usado no código!) |
| `realfft` | 0.5 | 3.5 |
| `ort` | 1.17 | 2.0.0-rc.15 |
| `rubato` | 0.15 | 0.15 ✓ |

**Nota**: `daspect` é usado em `audio/preprocessing.rs`, `audio/postprocessing.rs`, e `models/demucs.rs` mas **não está declarado** nas dependências do Cargo.toml.

---

## Status do Build

**Problema de Ambiente**: Projeto falha ao compilar no Windows devido a ferramentas de build C++ do Visual Studio ausentes (erros de linker para build scripts de proc-macro2, quote, libm, num-traits).

**Necessário**: Instalar workload "C++ build tools" no Visual Studio Installer.

Uma vez o linker funcionando, erros de compilação esperados:
1. `daspect` faltando nas dependências
2. `reqwest` + `tokio` faltando nas dependências
3. `models/demucs.rs` tem referências a variáveis não definidas
4. `thiserror` faltando para implementação de `errors.rs`
5. Todos os arquivos faltando precisam ser criados

---

## Próximos Passos - Prioridade

### Imediato (para desbloquear build):
1. Instalar Visual Studio C++ build tools
2. Adicionar dependências faltantes: `daspect`, `reqwest`, `tokio`, `thiserror`
3. Corrigir erros de compilação em `models/demucs.rs`

### Conclusão da Fase 1 (ordem das SPECs):
1. **SPEC_03**: Criar 8 arquivos .rs faltando (stubs)
2. **SPEC_04**: Implementar `lib.rs` completo com re-exports da API pública
3. **SPEC_05**: Implementar `main.rs` com init de logging + delegação para commands
4. **SPEC_06**: Criar `utils/errors.rs` com StemError, ResultExt, bail!
5. **SPEC_07**: Criar `utils/logging.rs` com variantes init_logging
6. **SPEC_08**: Criar `CLAUDE.md` com convenções do projeto
7. **SPEC_09**: Criar `.gitignore`
8. **SPEC_10**: Criar `.github/workflows/ci.yml`
9. **SPEC_11**: Criar `.rustfmt.toml`
10. **SPEC_12**: Rodar script de validação

### Fase 2 (Audio I/O & Preprocessing):
- Já amplamente implementado no módulo `audio/`
- Vai precisar integração com pipeline uma vez SPEC_03-07 completas

---

## Árvore de Arquivos (Atual)

```
Rustem/
├── Cargo.toml              ✅ Completo
├── Cargo.lock              ✅ Commitado
├── .gitignore              ❌ Faltando
├── .rustfmt.toml           ❌ Faltando
├── CLAUDE.md               ❌ Faltando
├── .github/workflows/      ❌ Faltando
├── SPEC_STEM_SEPARATOR.md  ✅ Existe
├── specs/phase_1/          ✅ 12 arquivos SPEC
├── .claude/commands/       ✅ implementar-spec.md
├── agents.md               ✅ Este arquivo
└── src/
    ├── main.rs             ⚠️ Mínimo (Hello world)
    ├── lib.rs              ⚠️ Mínimo (VERSION apenas)
    ├── audio/              ✅ Completo (4 arquivos)
    │   ├── mod.rs          ✅
    │   ├── io.rs           ✅
    │   ├── preprocessing.rs ✅
    │   └── postprocessing.rs ✅
    ├── models/             ✅ Completo (5 arquivos, 1 com bugs)
    │   ├── mod.rs          ✅
    │   ├── onnx.rs         ✅
    │   ├── demucs.rs       ⚠️ Erros de compilação
    │   ├── ensemble.rs     ✅
    │   └── registry.rs     ⚠️ Dependências faltando
    ├── separation/         ⚠️ 3/4 arquivos
    │   ├── mod.rs          ✅
    │   ├── stem.rs         ✅
    │   ├── config.rs       ✅
    │   └── pipeline.rs     ❌ Faltando
    ├── metrics/            ⚠️ 1/4 arquivos
    │   ├── mod.rs          ✅
    │   ├── sdr.rs          ❌ Faltando
    │   ├── sir.rs          ❌ Faltando
    │   └── sar.rs          ❌ Faltando
    ├── cli/                ⚠️ 1/3 arquivos
    │   ├── mod.rs          ✅
    │   ├── args.rs         ❌ Faltando
    │   └── commands.rs     ❌ Faltando
    └── utils/              ⚠️ 1/3 arquivos
        ├── mod.rs          ✅
        ├── errors.rs       ❌ Faltando
        └── logging.rs      ❌ Faltando
```

---

## Decisões Técnicas Principais (das SPECs)

| Decisão | Detalhe |
|---------|---------|
| Nome do crate | `Rustem` (package), biblioteca é `stem_separator` |
| Edition | 2024 |
| Licença | MIT OR Apache-2.0 |
| MSRV | 1.70+ |
| Sample rate | 44100 Hz padrão |
| STFT | Window: 4096, Hop: 1024 (75% overlap), janela Hann |
| Modelo | HTDemucs v4 (5 stems: vocals, drums, bass, piano, other) |
| ONNX Runtime | Carregamento dinâmico (`load-dynamic` feature) |
| GPU | Opcional via features `cuda` / `coreml` |
| Async | Apenas para download de modelos (tokio), DSP/inferência é sync (rayon) |
| Error handling | `thiserror` para tipos, `anyhow` para propagação |
| Logging | `tracing` com `RUST_LOG`, formato compacto |
| Testes | Módulos `#[cfg(test)]` inline, integração em `tests/` |

---

## Agentes/Workers para Implementação Paralela

Ao implementar specs restantes, considerar spawnar agentes para trabalho independente:

| Agente | Tarefa | Depende De |
|--------|--------|------------|
| Agente 1 | SPEC_03: Criar 8 .rs faltando (stubs) | SPEC_02 completo |
| Agente 2 | SPEC_04: Implementar lib.rs | SPEC_03 módulos existem |
| Agente 3 | SPEC_05: Implementar main.rs | SPEC_06, SPEC_07 |
| Agente 4 | SPEC_06: Criar utils/errors.rs | SPEC_03 |
| Agente 5 | SPEC_07: Criar utils/logging.rs | SPEC_03 |
| Agente 6 | SPEC_08: Criar CLAUDE.md | Nenhuma |
| Agente 7 | SPEC_09: Criar .gitignore | Nenhuma |
| Agente 7 | SPEC_10: Criar CI workflow | SPEC_02 |
| Agente 8 | SPEC_11: Criar .rustfmt.toml | Nenhuma |
| Agente 9 | Corrigir demucs.rs + adicionar deps faltantes | SPEC_02 |

---

## Notas para Implementação

1. **Seguir ordem das SPECs**: Cada SPEC tem dependências nas anteriores
2. **Usar skill implementar-spec**: Para cada SPEC, ler primeiro, propor plano, aguardar aprovação, implementar apenas o que está na spec
3. **Validar contra critérios de aceite**: Cada SPEC tem ACs específicos para verificar
4. **Não adiantar features**: Implementar apenas o que a SPEC atual requer
5. **Respeitar convenções CLAUDE.md**: Uma vez criado, seguir seus padrões de code style, error handling, testes, documentaçãofcc~