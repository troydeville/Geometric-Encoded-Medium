# G

## E

### M

---

#### 1  Project scaffolding 🧰

* [ ] **Create repo** (`git init`, add remote, pick licence → MIT?).
* [ ] `cargo new gem_aqm_demo --lib`; add **workspace** if you’ll break code into crates.
* [ ] Add CI (GitHub Actions) with `cargo check`, `clippy --all-targets`, `cargo test`, `cargo fmt -- --check`.

---

#### 2  Constants & core math crate `gem_core`

* [ ] Write `constants.rs` → pull values from CODATA 2022 once, expose as `const f64`.
* [ ] Implement dimension‑checked **type aliases** (`struct Quantity { val: f64, unit: Unit }` *or* use `uom` crate).
* [ ] Port your Mathematica formulas → Rust functions (`bohr_radius(mu)`, `vacuum_impedance()`, `gem_prediction(n, m)` …).
* [ ] Add **unit tests**: Bohr radius, fine‑structure α, G derived, electron energy ladder, etc.
  *Goal: cargo test must reproduce the numbers in the PDF (within chosen ε).*

---

#### 3  Observed‑data module `data`

* [ ] Create `observed.csv` with vetted lab values (Bohr‑radius, muonic hydrogen shift, Rydberg, planetary g, …).
* [ ] Write loader using `csv` crate → `Vec<Record>`.
* [ ] Integrity test: fails if any row has missing/invalid units.

---

#### 4  Comparison engine

* [ ] Function `compare(pred: Quantity, obs: Quantity) -> Diff { absolute, relative }`.
* [ ] Build **table model** (`Vec<Row>`) capturing label, prediction, observation, Δ.
* [ ] Add `assert!(relative < threshold)` tests for “nailed it” results (eg. muon shift).

---

#### 5  CLI prototype `bin/demo_cli.rs`

* [ ] Pretty‑print comparison table to stdout (`tabled` crate).
* [ ] Flag `--json` for machine parsable output.

---

#### 6  Interactive UI (choose one)

> Desktop = **egui** (native+web),  Web = **Yew**+WASM,  or **Tauri** hybrid.

* [ ] Scaffold app (`cargo run --package gem_ui`).
* [ ] **Slider / text‑box bindings** for:
  • principal quantum number n
  • particle mass m (dropdown: e⁻, μ⁻, p, custom)
  • optional Ω override
* [ ] Live compute prediction; update:
  • numeric fields
  • plot (use `plotters` for egui)
  • comparison badge (✔️ / ❌).
* [ ] Table pane with **chronological list** (auto‑append each time user hits “Record”).
* [ ] Sidebar toggle “Show derivations” with LaTeX rendered via `vsvg`/MathJax (if using web).

---

#### 7  Visualisations

* [ ] Energy level diagram (bar chart) vs. observed.
* [ ] Log‑log scatter of radius vs mass across particles.
* [ ] Option: animated slider scrubbing n to show Rydberg series converge.

---

#### 8  Testing & validation

* [ ] **Doctests** in formula docs.
* [ ] Criterion **benchmarks** for heavy numerics.
* [ ] Cross‑platform check (Windows, macOS, Linux).

---

#### 9  Docs & presentation material

* [ ] `README.md` – elevator pitch, build instructions, screenshot GIF.
* [ ] mdBook or Wiki section “Framework Walkthrough” linking to your PDFs.
* [ ] `slides/` → Reveal.js or Deck.rs; call core crate from WASM for live formulas inside slides.

---

#### 10  Packaging & release

* [ ] `cargo install --path .` builds CLI.
* [ ] `wasm-pack build --target web` bundles UI (if web).
* [ ] Tag `v0.1.0`, GitHub release with binaries & zip of web assets.
* [ ] Optional: Homebrew/Tap formula or Scoop manifest.

---

#### 11  Nice‑to‑haves

* [ ] “Export session” → JSON/CSV, “Share link” (if web) with encoded params.
* [ ] Nightly build that re‑pulls CODATA and re‑runs tests → badge on README.
* [ ] Embed **license & citation** info so researchers can reference your numbers.
* [ ] Provide minimal **Python** wrapper (pyo3) for notebooks.

---

### 🚀 Next step

Spin up the repo, knock out **Sections 1 & 2**, and you’ll have a solid backbone.  Once constants + predictions are rock‑solid, the UI layer becomes pure presentation ✨.

You’ve got this—one checkbox at a time!
