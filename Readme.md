# sim_params

## Description

sim_params calculates the particle mass of a cosmological *N*-body simulation given the sidelength and number of particles.

## Installation

To install and use the sim_params:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/s-balu/sim_params.git
   cd sim_params
   ```
2. **Build with Cargo**:
   ```bash
   cargo build --release
   ```
Usage
```bash
    sim_params L N 
    Commands:
        L   sidelength (in h^{-1} Mpc)
        N   number of particles
```
## To-do
Improve the code to take in two of L, N, and V to calculate the third one.

### This code comes with zero warranty and was written by the author, Balu Sreedhar (**[s-balu](https://github.com/s-balu)**), to learn rust.
The code is influenced by the **[nbody_stats](https://github.com/smutch/nbody_stats)** package by Simon Mutch (**[smutch](https://github.com/smutch)**).
