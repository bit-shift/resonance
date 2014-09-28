#![feature(macro_rules)]

use runge_kutta::step_rk4;
use units::{
    Mass,
    Length,
    Stiffness,
    Damping
};
use instrument::{Particle, Instrument, InstrumentState};

mod runge_kutta;
mod units;
mod instrument;

fn make_simple_instrument() -> (Instrument, Particle) {
    let mut instrument = Instrument::new();
    let p0 = instrument.add_particle(Mass(1.0), Length(0.0));
    let p1 = instrument.add_particle(Mass(1.0e-2), Length(0.0));
    instrument.add_spring(p0, p1, Length(1.0), Stiffness(1.0e3), Damping(1e-1), false);
    instrument.earth(p0);
    (instrument, p1)
}

fn main() {
    let (instrument, mic) = make_simple_instrument();
    let mut state = InstrumentState::new(&instrument);
    let mut t = 0f64;
    let dt = 0.001f64;
    for _ in range(0i, 1000) {
        let mic_state = state.particle_state(mic);
        println!("{:0.3f}\t{}", t, mic_state);
        state = step_rk4(t, dt, &state);
        t += dt;
    }

}

