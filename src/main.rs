#![feature(macro_rules)]

use runge_kutta::step_rk4;
use units::{
    Mass,
    Length,
    Velocity,
    Stiffness,
    Damping
};
use instrument::{Particle, Instrument, InstrumentState};

mod runge_kutta;
mod units;
mod instrument;

#[allow(dead_code)]
fn make_simple_instrument(mut instrument: Instrument) -> (Instrument, Particle) {
    let p_0 = instrument.add_particle(Mass(1.0), Length(0.0));
    let p_1 = instrument.add_particle(Mass(1.0e-2), Length(0.0));
    instrument.add_spring(p_0, p_1, Length(1.0), Stiffness(1.0e3), Damping(1e-1), false);
    instrument.earth(p_0);
    (instrument, p_1)
}

fn make_string(mut instrument: Instrument) -> (Instrument, Particle, Particle) {
    let string_mass = Mass(1e-2);
    let string_stiffness = Stiffness(1e-1);
    let string_damping = Damping(1e-3);
    let p_earth0 = instrument.add_particle(string_mass, Length(0.0));
    let p_target = instrument.add_particle(string_mass, Length(0.0));
    let p_pickup = instrument.add_particle(string_mass, Length(0.0));
    let p_earth1 = instrument.add_particle(string_mass, Length(0.0));
    instrument.earth(p_earth0);
    instrument.earth(p_earth1);
    instrument.add_chain(p_earth0, p_target, 5, string_mass, string_stiffness, string_damping);
    instrument.add_chain(p_target, p_pickup, 5, string_mass, string_stiffness, string_damping);
    instrument.add_chain(p_pickup, p_earth1, 5, string_mass, string_stiffness, string_damping);
    (instrument, p_target, p_pickup)
}

fn main() {
    let hammer_mass = Mass(0.1);
    let hammer_stiffness = Stiffness(1e1);

    let instrument = Instrument::new();
    let (mut instrument, p_target, p_pickup) = make_string(instrument);
    let p_hammer = instrument.add_particle(hammer_mass, Length(0.0));
    instrument.add_spring(p_hammer, p_target, Length(0.0), hammer_stiffness, Damping(0.0), true);

    let mut state = InstrumentState::new(&instrument);
    state.trigger_hammer(p_hammer, p_target, Velocity(1.0));
    let mut t = 0f64;
    let dt = 0.001f64;
    for _ in range(0i, 1000) {
        let pickup_state = state.particle_state(p_pickup);
        let hammer_state = state.particle_state(p_hammer);
        println!("{:0.3f}\t{}\t{}", t, pickup_state, hammer_state);
        state = step_rk4(t, dt, &state);
        t += dt;
    }

}

