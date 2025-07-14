# AGENTS.md

> Istruzioni per agenti AI (es. Codex, GPT, Copilot) che interagiscono con questa repository.
> Questo documento definisce convenzioni, struttura e obiettivi del progetto `opherast`.

---

## 📌 Contesto del Progetto

`opherast` è un backend framework modulare scritto in **Rust**, ispirato a **Laravel** e **Symfony**, con un'architettura **feature-first** e un CLI associato (`cargo opherast`). È pensato per creare backend REST API moderne, con supporto multi-database e struttura scalabile.

---

## 📂 Struttura del Repository

Il repository è organizzato come un **workspace Cargo** con i seguenti membri:

- `opherast-framework/` — il core del framework, contiene i moduli runtime, le utility, il supporto ORM (SeaORM) e le astrazioni condivise.
- `opherast-cli/` — il tool `cargo opherast` (aka `xtask`) per generare scaffolding, feature e comandi.
- `opherast/` — il progetto app/demo principale, che usa il framework.
- `.cargo/config.toml` — contiene alias come `cargo opherast`.

---

## 🧠 Convenzioni Chiave

### Feature Modules

Ogni **feature** (es. `user`, `project`, `auth`) ha la seguente struttura generata automaticamente:

```
features/
  └── <feature_name>/
        ├── mod.rs
        ├── handler.rs     # API layer (Axum handlers)
        ├── service.rs     # Business logic
        ├── repository.rs  # DB logic
        ├── model.rs       # Entity structs (SeaORM)
        └── dto.rs         # Input/output structs
```

Gli agenti devono mantenere questa struttura e registrare handler e servizi nel `mod.rs` della feature.

### CLI: `cargo opherast`

Il CLI (`opherast-cli`) usa un sistema di **comandi modulari** (in `commands/`) con **registrazione automatica** via `build.rs`. Ogni comando è un file `.rs` che implementa `Command`.

Esempio:

```rust
// commands/generate.rs
pub struct Generate;

impl Command for Generate {
    fn name(&self) -> &'static str { "generate" }
    fn run(&self, args: &[String]) -> eyre::Result<()> { ... }
}
```

---

## ⚙️ Tecnologie e Scelte Architetturali

- 🧱 ORM: https://www.sea-ql.org/SeaORM/
- 🧵 Concurrency: async/await (tokio)
- 🧪 Error handling: `eyre` (no `anyhow`)
- 📦 Crate utility comune: `opherast-framework`
- 🛠 Comandi CLI: `opherast-cli`, con macro di registrazione
- 🌐 API: basato su `axum`, middleware personalizzati

---

## 🧠 Agenti: Cosa Sapere

### ✅ Fai

- Usa la struttura `features/<name>/` per nuove funzionalità.
- Usa le macro già presenti (es. derive personalizzati).
- Registra i nuovi handler nei mod.rs.
- Mantieni separazione chiara tra handler/service/repository.

### 🚫 Non fare

- Non usare `anyhow` — usa `eyre`.
- Non scrivere codice monolitico: rispetta la modularità.
- Non inserire logica di business negli handler.
- Non accedere al DB direttamente fuori dai `repository.rs`.

---

## 🧪 Testing

(TODO) I test saranno collocati in `tests/` oppure per-feature. Per ora usa `cargo test` nei moduli.

---

## 📚 Glossario

- **Feature**: una unità funzionale isolata (es. auth, user).
- **Handler**: endpoint HTTP, spesso Axum router.
- **Service**: logica applicativa.
- **Repository**: accesso ai dati, via SeaORM.
- **DTO**: oggetti per input/output (API-facing).
- **DbModel**: derive macro personalizzata per le entità DB.

---

## 🤖 Note per Evoluzione

In futuro saranno aggiunti:
- Supporto Redis, SQLite, MySQL tramite `DB_DRIVER`.
- Sistema di autorizzazione PBAC con Casbin.
- Decoratori e macro personalizzati per middleware, validation, etc.

---
