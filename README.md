# Geometric Encoded Medium (GEM): The Vacuum Structured as a Bound Medium

📄 **[Read the White Paper (PDF)](GEM_White_Paper.pdf)** (v0.2 – November 2025)
🔗 https://github.com/troydeville/Geometric-Encoded-Medium


GEM is a unified physical framework where all observable phenomena arise from exact geometric relationships encoded in a constrained vacuum substrate. All fields, forces, and particles emerge from the resonance of mass and charge within the medium’s encoded structure.

Inspired by pattern recognition in constants and Planck-scale geometry, GEM predicts binding energies, radii, gravitational forces, and more across scales—from electrons to stars—with striking accuracy. New in v0.2: Universal vacuum specific charge Λ, complex phase topology for black holes, and macroscopic predictions.

This repo is the complete archive of derivations, code, and docs. It's ongoing—strong-field cases have ~10^{-9} residuals being refined.

---

## Repository Structure

- **docs/**  
  Core documents, including white papers (v0.1 and v0.2), drafts, and reference tables.

- **figures/**  
  Surface geometry plots, 3D visualizations, and rendered outputs from GEM curvature fields (e.g., gem_curvature_top.png).

- **gem_engine/**  
  Rust library for core computations. Includes constants.rs (with Λ, α, etc.), quantum.rs (predictions for binding energies, Lamb shifts, gravity), helper.rs, lib.rs, and data/observed.csv for validation. Run `cargo test` to verify predictions.

- **v0/**  
  Early version files: CONTRIBUTING.md, Glossary.md, derivations (e.g., Derivation_Acceleration_Function.md), examples (e.g., Acceleration_and_Gravity.md, Mathematica notebooks), journal (e.g., Draft_Paper.md, Research_Notes.md), and Theory (e.g., Constants.md, Mathematica CONSTANTS.nb).

- **v1_conceptual/**  
  Conceptual explorations: old.txt (legacy notes), rust_code.txt (early Rust snippets), test.tex (LaTeX tests), and notebooks/ (discovery_notebook.pdf with README.md).

- **Root Files**: .gitignore, GEM_White_Paper.pdf (exported v0.2), LICENSE (GPL v3), README.md, TODO.md.

---

## 📘 Documents & History

- **GEM White Paper v0.2 (November 2025)** – Updated summary with universal Λ, complex phase, macro gravity.  
  [View docs/v0.2 - GEM_White_Paper.md](docs/v0.2 - GEM_White_Paper.md) or [PDF export](GEM_White_Paper.pdf)

- **GEM 0.1.0 Draft** – Original exploratory doc with derivations.  
  [View docs/v0.1 - GEM_0.1.0_Draft.pdf](docs/v0.1 - GEM_0.1.0_Draft.pdf)

- **Discovery Notebook** – Raw Mathematica explorations.  
  [View v1_conceptual/notebooks/discovery_notebook.pdf](v1_conceptual/notebooks/discovery_notebook.pdf)

This project evolved from April 2025 (v0.1) to include new insights like the universal constant Λ ≈ 8.209 × 10⁹ C/kg and black hole phase flips. More simulations coming.

---

## Quick Start (Rust Engine)

```bash
cd gem_engine
cargo test  # Verifies predictions (Bohr radius, muonic shift, Sun-Earth force, etc.)
cargo run   # (If you add a main.rs CLI demo)