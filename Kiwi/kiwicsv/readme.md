# KiwiCSV 🥝 (WIP)

A small, opinionated CSV parser written in Rust that converts CSV text into a typed 2D structure (`KiwiCSV`) using custom inference/types (`KiwiType`) and supports **round-trip serialization** back to CSV.

> This is currently a **library-first** project. A CLI (via `clap`) is planned later.

---

## Features ✨

- Parses CSV-ish text into a structured representation:
  - `KiwiCSV { header, content, footer }`
- Custom type inference into:
  - `KiwiType::Int(isize)`
  - `KiwiType::Float(KiwiFloat)`
  - `KiwiType::String(String)`
  - `KiwiType::NaN`
  - `KiwiType::Unknown` / `Other { ... }` (depending on your current enum)
- Tokenization configurable via `KiwiTokenizer`:
  - custom delimiter (`,` by default)
  - optional trimming
  - optional filling (pad short rows)
- Supports **round-trip testing**:
  - CSV → KiwiCSV → CSV → KiwiCSV should preserve structure (within current formatting rules)

---

## Non-goals (for now) 🚫

This is a **simple parser** right now, not a full RFC-compliant CSV engine.

Currently NOT supported (yet):

- Quoted fields (e.g. `"hello, world"`)
- Escapes inside quoted strings
- Multiline quoted fields
- Comments / metadata lines
- Arbitrary header discovery (header is assumed to be the first row if enabled)

---

## Data Model 🧠

### `KiwiCSV`

- `header: Vec<String>`
- `content: Vec<Vec<KiwiFruit>>`
- `footer: Option<Vec<KiwiFruit>>` (or whatever your latest type is)

### `KiwiFruit`

Represents a cell result:

- `KiwiFruit::Type(KiwiType)` for successful typed parse
- `KiwiFruit::Error(KiwiError)` for parse/inference failures

### `KiwiSettings`

Controls whether the CSV is expected to include header/footer rows:

- `header: bool`
- `footer: bool`

If `settings.header == true`, the **first row** is treated as header.
If `settings.footer == true`, the **last row** is treated as footer.

---

## Install / Use 📦

Add this crate as a dependency (local path during development):

```toml
[dependencies]
kiwicsv = { path = "../kiwicsv" }
```
