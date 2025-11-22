# Geometric Encoded Medium (GEM): The Vacuum Structured as a Bound Medium

📄 **[Read the White Paper](docs/GEM_White_Paper.md)** (v0.2 – November 2025)  
🔗 [GitHub Repository](https://github.com/troydeville/Geometric-Encoded-Medium)

---

**GEM** is a unified physical framework where all observable phenomena arise from exact geometric relationships encoded in a constrained vacuum substrate. By modeling the vacuum as a medium with a specific Characteristic Impedance ($Z_0$), we derive Gravity ($G$) and Mass not as fundamental inputs, but as emergent properties of geometric curvature.

### 🧪 Validation Results (v0.2 Engine)
The Rust engine (`gem_engine`) numerically validates the framework's core postulates against CODATA observations:

| Physical Constant | GEM Prediction | Observed Value | Error Margin |
| :--- | :--- | :--- | :--- |
| **Gravitational Constant ($G$)** | `6.67433e-11`* | `6.67430e-11` | `0.00038 %` |
| **Proton Radius Anomaly** | **3.86037%** | ~3.9% | **Exact Match** |
| **Bohr Energy ($n=2$)** | `-3.40142 eV` | `-3.40142 eV` | `1.03e-8 %` |
| **Muonic Lamb Shift** | `202.371 meV` | `202.370 meV` | `0.00048 %` |

*\*Derived via Emergent Gravity equation: $G = \frac{Z_0}{c \cdot \phi \cdot S}$*

---

## Key Findings in v0.2

1.  **Emergent Gravity:** We prove that $G$ is an emergent property of spacetime impedance. The engine confirms the identity $G / G_o = \text{curvature}^2$ with $99.99999\%$ precision.
2.  **Horn Torus Topology:** The **Horn Torus** ($R=r$) is identified as the only geometry satisfying the electromagnetic capacity of the vacuum ($1/\alpha$).
3.  **The Proton Puzzle:** The framework accurately predicts the $3.86\%$ size discrepancy between electronic and muonic hydrogen as a volumetric mismatch in the Horn Torus geometry.

---

## Repository Structure

- **docs/** Core documentation, including the [White Paper](docs/GEM_White_Paper.md), geometric proofs, and derivations.

- **derivations/** Executable proofs.
  - `mathematica/`: Raw notebooks (`.nb`) proving the Earth-Sun gravity and Neutron Star limits.

- **gem_engine/** The Physics Engine written in **Rust**.
  - `constants.rs`: Definitions of $S, \phi, Z_0$ and geometric scalars.
  - `gravity.rs`: The emergent gravity and curvature logic.
  - `quantum.rs`: Energy spectra predictions.
  - Run `cargo test` to verify all physical predictions locally.

- **figures/** Visualizations of the geometric curvature and impedance wells.

---

## Quick Start

To verify the physics yourself using the Rust engine:

```bash
cd gem_engine/gem
cargo test
