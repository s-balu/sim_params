mod units;
mod constants;
mod cosmology;

use crate::constants::PI;
use crate::constants::G;

use crate::cosmology::Planck2020::H0;
use crate::cosmology::Planck2020::Om0;
use crate::units::METRES_PER_PC;
use crate::units::KILOGRAMS_PER_SOLAR_MASS;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let L = &args[1].parse::<f64>().unwrap();
    let N = &args[2].parse::<f64>().unwrap();

    let _H0: f64 = H0  * 1000.0 / (1e6 * METRES_PER_PC); // this is in SI now.

    let mut rho_crit0: f64  = (3.0 * _H0.powf(2.0))/(8.0*PI*G); // this is in SI unit: h^2 kg m^-3 now.
    // println!("rho_crit0: {}", rho_crit0*1e26); // should be ~ 1.88e-26.
    
    rho_crit0 = rho_crit0 * ( (1e6 * METRES_PER_PC).powf(3.0) / KILOGRAMS_PER_SOLAR_MASS ); // unit: h^2 M_Sun Mpc^-3

    let m_part = rho_crit0 * Om0 * L.powf(3.0) / N.powf(3.0);
    
    println!("Sidelength, L: {} h^-1 Mpc", L);
    println!("Number of particles: {}^3", N);
    println!("{}", format!("Particle mass: {m_part:.4e} h^-1 M_Sun"));

}
