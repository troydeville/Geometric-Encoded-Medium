import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

# ==========================================
# 1. GEM CONSTANT DERIVATION (Exact)
# ==========================================

# Base SI Constants
c = 299792458.0
h = 6.62607015e-34
e = 1.602176634e-19
pi = np.pi

# GEM Alpha Derivation (Matching constants.rs)
# αγ = 4580703784999263461548761·π
alpha_gamma = 4580703784999263461548761.0 * pi
# αδ = 1972044687500000000000000000
alpha_delta = 1972044687500000000000000000.0
alpha = alpha_gamma / alpha_delta

# GEM Impedance Factors
# Γ = (α h) / (2π c)
gamma_const = (alpha * h) / (2 * pi * c)

# Ω = 100·(4π)^(1/8)
omega = 100.0 * (4 * pi)**(1/8)

# Φ = Γ / e²
phi = gamma_const / (e**2)

# DERIVED G (The Geometric Constant)
# G = (4π x Φ)/(Ω x Ω)
G_GEM = (4 * pi * phi) / (omega**2)

print(f"------------------------------------------------")
print(f"GEM Derived G: {G_GEM:.14e}")
print(f"CODATA G     : 6.67430000000000e-11")
print(f"------------------------------------------------")

# ==========================================
# 2. PLOTTING THE CURVATURE WELL
# ==========================================

# Epsilon0 derived from c and Z0 logic (or standard for plot scaling)
epsilon_0 = 8.854187817e-12 

# Earth Mass for scale visualization
M = 5.972e24 
# Schwarzschild Radius using GEM Derived G
Rs = (2 * G_GEM * M) / (c**2)

def calculate_curvature_potential(x, y):
    r = np.sqrt(x**2 + y**2)
    
    # Handle singularity (avoid div by zero)
    r[r < Rs] = Rs 
    
    # The GEM Derivation geometric term: Sqrt[1 - Rs/d]
    # We map the inverse to visualize the well
    geometric_term = np.sqrt(1 - (Rs / (r + Rs))) 
    
    # The curvature intensity
    impedance_curvature = 1 / geometric_term
    return impedance_curvature

# Grid setup
grid_size = 100
x = np.linspace(-50, 50, grid_size)
y = np.linspace(-50, 50, grid_size)
X, Y = np.meshgrid(x, y)

# Calculate Z (Curvature)
Z = calculate_curvature_potential(X, Y)

# Plotting
fig = plt.figure(figsize=(12, 8))
ax = fig.add_subplot(111, projection='3d')

# Plot surface with plasma map (High energy/curvature = Yellow, Low = Purple)
surf = ax.plot_surface(X, Y, -Z, cmap='magma', edgecolor='none', alpha=0.9)

# Styling
ax.set_title(f"Geometric Encoded Medium\nImpedance Well (G derived from $\Omega$)", fontsize=14, color='white')
ax.set_axis_off() 
fig.patch.set_facecolor('#0d1117') # GitHub Dark Dimmed
ax.set_facecolor('#0d1117')

# Save
plt.savefig('../docs/figures/gem_curvature_3d.png', dpi=300, facecolor='#0d1117')
print("Plot saved to docs/figures/gem_curvature_3d.png")