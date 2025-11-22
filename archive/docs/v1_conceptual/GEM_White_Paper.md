# Geometric Encoded Medium (GEM): A Predictive Vacuum Framework Based on Planck Geometry

**Troy Deville**  
*Electrical Engineer, EI & Independent Researcher*  
Version 0.1 – April 2025

---

## Abstract

This paper introduces the *Geometric Encoded Medium (GEM)*—a purely algebraic framework grounded in geometric resonance and vacuum structure. Developed through pattern recognition in physical constants and Planck-scale units, GEM reconstructs fundamental quantities such as the fine-structure constant, classical particle radii, and gravitational parameters. Using only basic constants and fractional relationships, the model predicts binding energies for elementary particles and atomic systems with striking accuracy, while suggesting a geometric origin for gravitational curvature. This intuitive approach offers a complementary path to understanding mass, charge, and force without relying on advanced mathematics or field theory, and is accessible to engineers, physicists, and general readers alike.

---

## 1. Introduction

Physics often advances through complex equations—but sometimes, geometry and structure reveal deeper truths. The *Geometric Encoded Medium (GEM)* is based on a deceptively simple idea: that the vacuum is not empty, but a structured, quantized medium in which mass and charge arise from geometric resonance.

Guided by the axiom that **resonance is geometric action**, this framework reconstructs familiar constants and observables—including the fine structure constant \( \alpha \), gravitational constant \( G \), and binding energies for fundamental particles—entirely from dimensionless ratios of Planck units and exact algebraic expressions. No Lagrangians, field equations, or assumptions about quantization are used.

This paper is both an intuitive map and technical log of the geometric structure that may underlie the Standard Model and General Relativity. What follows is a snapshot of predictions, geometrical structures, and numerical outputs validated against experimental constants using simple Mathematica code and dimensional logic.

---

## 2. Core Framework

At the heart of GEM is a relationship between physical constants that defines the vacuum as a bound, geometric medium. The resonance structure of this medium encodes mass and charge in purely algebraic terms.

### 2.1 Vacuum Constants and Key Expressions

Let:
- \( c \): speed of light  
- \( h \): Planck constant  
- \( e \): elementary charge  
- \( \mu_0 \), \( \epsilon_0 \): vacuum permeability and permittivity  
- \( \alpha \): fine-structure constant  
- \( l_P \), \( m_P \): Planck length and mass

The fundamental vacuum coupling constant is:

\[
\Gamma = \frac{e^2}{4 \pi c^2 \epsilon_0} = \frac{e^2 \mu_0}{4 \pi} = \alpha l_P m_P
\]

This expression forms the algebraic bridge between electric charge and vacuum geometry. Masses and radii are then derived from integer fractions of Planck units via:

\[
n = \frac{m}{m_P}, \quad r_n = \frac{l_P}{\Lambda(n)}
\]

Where \( \Lambda(n) \) is a geometric scaling factor derived from fine-structure partitions. Binding energies, curvature, and acceleration are subsequently functions of \( n \), \( m_P \), and \( l_P \).

---

### 2.2 Geometric Mass-Radius Constraint

The fundamental geometric constraint central to GEM’s vacuum structure is:

\[
\frac{M}{\Gamma} \left(R - \frac{2GM}{c^2}\right) = 1
\]

This condition relates mass, radius, and vacuum curvature and defines how Planck-scale units compress the vacuum medium to create mass-energy structure.

---

### 2.3 Fine Structure Encoding from Vacuum Compression

One of the key expressions in GEM emerges from the geometry of vacuum compression, revealing a structure for the fine-structure constant \( \alpha \) based on Planck-scale quantities.

We begin with the geometric resonance condition:

\[
\frac{M}{\Gamma} \left(R - \frac{2GM}{c^2}\right) = 1
\]

Let:

- \( M = n \cdot m_P \) (a multiple of the Planck mass),
- \( \Gamma = \alpha m_P l_P \),
- \( R = \frac{l_P}{\Lambda_\alpha(n)} \)

Substituting:

\[
\frac{n m_P}{\Gamma} \left( \frac{l_P}{\Lambda_\alpha(n)} - \frac{2 G n m_P}{c^2} \right) = 1
\]

Rewriting the left-hand side to isolate \( \Lambda_\alpha(n) \) yields:

\[
\Lambda_\alpha(n) = \frac{l_P}{R} = \frac{\alpha_\delta \cdot n}{2 \alpha_\delta \cdot n^2 + \alpha_\gamma}
\]

Where the parameters \( \alpha_\gamma \) and \( \alpha_\delta \) are:

\[
\alpha_\gamma = 4580703784999263461548761 \cdot \pi \\
\alpha_\delta = 1972044687500000000000000000
\]

Thus, the fine structure constant is expressed as:

\[
\alpha = \frac{\alpha_\gamma}{\alpha_\delta}
\]

#### 2.3.1 Remarks

The structure of \( \Lambda_\alpha(n) \) encodes \( \alpha \) in a scaled rational form:

- The **numerator** matches the denominator of \( \alpha \), scaled by \( n \),
- The **denominator** has two terms:
  - One is half of the scaled denominator of \( \alpha \) with \( n^2 \),
  - The other is the original numerator of \( \alpha \).

This suggests that GEM encodes the fine structure constant not as a postulate but as a geometric consequence of vacuum resonance involving Planck quantities.


---
## 3. Key Predictions from GEM

The GEM framework yields high-accuracy predictions across multiple domains:

---

### Table 1. GEM-Predicted Binding Energy Levels (eV)

| Particle | Level 1 | Level 2 | Level 3 | Level 4 | Level 5 | Level 6 | Level 7 | Level 8 | Level 9 | Level 10 |
|----------|---------|---------|---------|---------|---------|---------|---------|---------|----------|-----------|
| Muon     | 2813.23 | 703.31  | 312.58  | 175.83  | 112.53  | 78.15   | 57.41   | 43.96   | 34.73    | 28.13     |
| Proton   | 24982.13 | 6245.53 | 2775.79 | 1561.38 | 999.29  | 693.95  | 509.84  | 390.35  | 308.42   | 249.82    |
| Neutron  | 25016.57 | 6254.14 | 2779.62 | 1563.54 | 1000.66 | 694.90  | 510.54  | 390.88  | 308.85   | 250.17    |
| Tau      | 47310.09 | 11827.52| 5256.68 | 2956.88 | 1892.40 | 1314.17 | 965.51  | 739.22  | 584.08   | 473.10    |
| Deuteron | 49930.75 | 12482.69| 5547.86 | 3120.67 | 1997.23 | 1386.97 | 1018.99 | 780.17  | 616.43   | 499.31    |
| Quark?   | 82539.60 | 20634.90| 9171.07 | 5158.72 | 3301.58 | 2292.77 | 1684.48 | 1289.68 | 1019.01  | 825.40    |

---

### Table 2. GEM-Derived Geometric Radii (meters)

| Particle | Radius 1 | Radius 2 | Radius 3 | Radius 4 | Radius 5 | ... |
|----------|-----------|-----------|-----------|-----------|-----------|-----|
| Muon     | 2.56e-13  | 1.02e-12  | 2.30e-12  | 4.09e-12  | 6.40e-12  | ... |
| Proton   | 2.88e-14  | 1.15e-13  | 2.59e-13  | 4.61e-13  | 7.20e-13  | ... |
| Neutron  | 2.88e-14  | 1.15e-13  | 2.59e-13  | 4.60e-13  | 7.20e-13  | ... |
| Tau      | 1.52e-14  | 6.09e-14  | 1.37e-13  | 2.43e-13  | 3.80e-13  | ... |
| Deuteron | 1.44e-14  | 5.77e-14  | 1.30e-13  | 2.31e-13  | 3.60e-13  | ... |
| Quark?   | 8.72e-15  | 3.49e-14  | 7.85e-14  | 1.40e-13  | 2.18e-13  | ... |

---

---

### Table 3. Rydberg Wavelengths and Frequencies (Hydrogen Series)

| Level (n) | Wavelength (m) | Frequency (Hz) | Angular Frequency (rad/s) |
|-----------|----------------|----------------|----------------------------|
| 1         | 9.11e-8        | 3.29e15        | 2.07e16                    |
| 2         | 3.65e-7        | 8.22e14        | 5.17e15                    |
| 3         | 8.20e-7        | 3.66e14        | 2.30e15                    |
| 4         | 1.46e-6        | 2.06e14        | 1.29e15                    |
| ...       | ...            | ...            | ...                        |

---

### Table 4. Surface Gravity from GEM Curvature

| Object        | Mass (kg)         | Radius (m)     | GEM Gravity \(g\) (m/s²) | Observed Gravity |
|---------------|-------------------|----------------|--------------------------|------------------|
| Earth         | 5.972e24          | 6.371e6        | ~9.820                   | 9.820 (mean)     |
| Sun           | 1.989e30          | 6.957e8        | ~274.2                   | 274              |
| Neutron Star  | 2.784e30          | 1.0e5          | ~2.09e12                 | N/A              |

These gravitational accelerations are derived using the GEM vacuum compression radius and curvature scaling factor \( k \), as shown in the Mathematica implementation.

### 3.1 Example: Hydrogen Energy Level Transition

To demonstrate the predictive power of GEM, we compute the energy difference between the \( n = 2 \) and \( n = 1 \) states in hydrogen using the following expression:

\[
E_n = -\frac{c^2 m \alpha^2 Z^2}{2 n^2}
\]

Where:

- \( c \) is the speed of light,
- \( m \) is the electron (or reduced) mass,
- \( \alpha \) is the fine-structure constant,
- \( Z \) is the atomic number (1 for hydrogen),
- \( n \) is the energy level.

The transition energy between two levels is then:

\[
\Delta E = E_{n=2} - E_{n=1}
\]

Evaluated numerically in Mathematica:

```mathematica
En[m_, n_] := UnitConvert[-(α^2 / (2 n^2)) c^2 m, "Electronvolts"];
Ezn[m_, n_, Z_] := UnitConvert[-((Z^2 α^2)/(2 n^2)) c^2 m, "Electronvolts"];
SetPrecision[Ezn[me, 2, 1] - Ezn[me, 1, 1], 32]
```

Output:

```mathematica
10.204269844943976422346034492749 eV
```

This result aligns with the observed Lyman-𝛼 transition energy in hydrogen (≈ 10.2 eV), showing that GEM’s geometric vacuum structure naturally reproduces known quantum energy levels.

---

## 4. Voltage, Acceleration, and Gravity Emergence

One of the most novel results of GEM is the emergence of gravity from a coupling between potential difference and geometric acceleration:

\[
V = \frac{\alpha}{2\pi} \cdot \frac{chM}{\Gamma Q}, \quad
a = \frac{\alpha}{2\pi} \cdot \frac{chM^2}{\Gamma^2}
\]

Combining gives:
\[
\frac{V}{a} = \frac{\Gamma}{MQ} \cdot \frac{c^2}{\alpha} \Rightarrow G
\]

This suggests that **voltage per unit acceleration**—applied through GEM's framework—yields the gravitational constant \( G \), a stunning result that unifies electromagnetic and gravitational structure.

---

## 5. Geometry and Visualizations

The GEM vacuum structure can be visualized as a resonant geometric surface—quantized curvature at Planck-scale resolution.

### Figure 1. Top-Down View of GEM Curvature

![GEM Vacuum Resonance Top View](figures/gem_curvature_top.png)

*This image shows the geometric curvature at 1 Planck mass and 1 Planck length. The central disk and radial "petals" reveal the vacuum's encoded resonance pattern.*

---

### Figure 2. 3D Perspective of Curvature Field

![GEM Vacuum Resonance 3D](figures/gem_curvature_3d.png)

*The pronounced central curvature well and rippling structure represent mass encoded as geometric compression. This is the visual core of the GEM hypothesis.*

---

## 6. Conclusion

GEM offers a novel path: building the universe from constants and resonance alone. Without Lagrangians or quantum fields, the model derives:

- The fine-structure constant
- Binding energy spectra of leptons and baryons
- Rydberg states of hydrogen
- Gravitational scaling for Earth and stellar bodies

Its simplicity—coupled with predictive power—suggests that geometry itself may encode physical law.

Whether GEM is a toy model, pre-quantum geometry, or the seed of something deeper, it invites curiosity, testing, and creative iteration. This work is shared openly in the spirit of exploration.

---

## Suggested Citation

**Troy Deville.** *Geometric Encoded Medium (GEM): A Predictive Vacuum Framework Based on Planck Geometry_. GitHub repository, <https://github.com/troydeville/Geometric-Encoded-Medium>, 2025.

**BibTeX:**
    @misc{deville2025gem,
      author = {Deville, Troy},
      title = {Geometric Encoded Medium (GEM): A Predictive Vacuum Framework Based on Planck Geometry},
      year = {2025},
      howpublished = {\url{https://github.com/troydeville/Geometric-Encoded-Medium}},
      note = {Version 0.1, accessed April 2025}
    }
