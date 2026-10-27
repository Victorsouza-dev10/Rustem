# Spec 11: rustfmt.toml - Formatação de Código

**Fase**: 1 - Setup & Fundação  
**Componente**: Configuração rustfmt (`.rustfmt.toml`)  
**Duração estimada**: 15 min  
**Dependência**: SPEC_01_CARGO_INIT

---

## Objetivo
Configurar formatação de código consistente com `rustfmt` usando configuração personalizada.

---

## Arquivo: `.rustfmt.toml`

```toml
# rustfmt Configuration for stem-separator
# Run with: cargo fmt --all

edition = "2021"
max_width = 100
tab_spaces = 4
hard_tabs = false
newline_style = "Unix"
use_small_heuristics = "Max"
reorder_imports = true
group_imports = "StdExternalCrate"
imports_granularity = "Crate"
normalize_comments = true
format_code_in_doc_comments = true
format_strings = true
wrap_comments = true
```

---

## Configurações Explicadas

| Configuração | Valor | Descrição |
|--------------|-------|-----------|
| `edition` | `"2021"` | Rust edition |
| `max_width` | `100` | Largura máxima da linha (100 chars) |
| `tab_spaces` | `4` | Espaços por tab (4 spaces) |
| `hard_tabs` | `false` | Usar espaços, não tabs |
| `newline_style` | `"Unix"` | LF line endings |
| `use_small_heuristics` | `"Max"` | Heurísticas agressivas para linhas curtas |
| `reorder_imports` | `true` | Reordenar imports alfabeticamente |
| `group_imports` | `"StdExternalCrate"` | Agrupar: std → external → local |
| `imports_granularity` | `"Crate"` | Um use por crate |
| `normalize_comments` | `true` | Normalizar comentários |
| `format_code_in_doc_comments` | `true` | Formatar código em doc comments |
| `format_strings` | `true` | Formatar strings longas |
| `wrap_comments` | `true` | Quebrar comentários longos |

---

## Configurações Avançadas (Opcionais)

```toml
# Formatting
chain_width = 60
single_line_if_else_max_width = 50
attr_fn_like_width = 70
struct_lit_width = 80
fn_single_line = false
force_explicit_abi = true
force_multiline_blocks = true
empty_item_single_line = true

# Imports
imports_indent = "Block"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
reorder_imports = true
reorder_modules = true

# Formatting
indent_style = "Block"
indent_block = 4
```

---

## Critérios de Aceite

| ID | Critério | Verificação |
|----|----------|-------------|
| AC-11.1 | Arquivo existe | `ls -la .rustfmt.toml` |
| AC-11.2 | `cargo fmt --check` passa | `cargo fmt --all -- --check` |
| AC-11.2 | Largura 100 chars | Linhas não passam 100 chars |
| AC-11.3 | Imports organizados | `cargo fmt` reorganiza imports |

---

## Checklist

- [ ] `.rustfmt.toml` criado na raiz
- [ ] Configurações principais definidas
- [ ] `cargo fmt --all -- --check` passa
- [ ] `cargo fmt` formata sem erros

---

## Verificação

```bash
# Verificar formatação
cargo fmt --all -- --check

# Aplicar formatação
cargo fmt --all

# Verificar largura de linha
cargo fmt --all -- --check 2>&1 | grep -i "width\|line" || echo "OK"
```

---

## Notas

- Opcional: rustfmt usa padrões sensatos por padrão
- Arquivo só necessário se quiser customizar
- `cargo fmt` usa `.rustfmt.toml` automaticamente se existir
- `max_width = 100` é padrão da equipe (não 80 ou 120)