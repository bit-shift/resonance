#![feature(macro_rules)]

extern crate openal;

use runge_kutta::step_rk4;
use units::{
    Mass,
    Length,
    Velocity,
    Stiffness,
    Damping
};
use instrument::{Particle, Instrument, InstrumentState};
use std::i16;
use openal::{al, alc};
use std::io::timer::sleep;
use std::time::duration::Duration;

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

fn make_string() -> (Instrument, Particle, Particle, Particle) {
    let hammer_mass = Mass(0.05);
    let hammer_stiffness = Stiffness(1e-1);
    let string_mass = Mass(1e-2);
    let string_stiffness = Stiffness(3e5);
    let string_damping = Damping(0.5);

    let mut instrument = Instrument::new();
    let p_earth0 = instrument.add_particle(string_mass, Length(0.0));
    let p_target = instrument.add_particle(string_mass, Length(0.0));
    let p_pickup = instrument.add_particle(string_mass, Length(0.0));
    let p_earth1 = instrument.add_particle(string_mass, Length(0.0));
    let p_hammer = instrument.add_particle(hammer_mass, Length(0.0));
    instrument.earth(p_earth0);
    instrument.earth(p_earth1);
    instrument.add_chain(p_earth0, p_target, 15, string_mass, string_stiffness, string_damping);
    instrument.add_chain(p_target, p_pickup, 15, string_mass, string_stiffness, string_damping);
    instrument.add_chain(p_pickup, p_earth1, 15, string_mass, string_stiffness, string_damping);
    instrument.add_spring(p_hammer, p_target, Length(0.0), hammer_stiffness, Damping(0.0), true);
    (instrument, p_hammer, p_target, p_pickup)
}

fn main() {
    let gain_out = 1000.0;
    let hammer_velocity =  Velocity(1e2);
    let (instrument, p_hammer, p_target, p_pickup) = make_string();

    let mut state = InstrumentState::new(&instrument);
    state.trigger_hammer(p_hammer, p_target, hammer_velocity);
    let next_sample = |t, dt| -> f64 {
        let pickup_state = state.particle_state(p_pickup);
        //let hammer_state = state.particle_state(p_hammer);
        //println!("{:0.3f}\t{}\t{}", t, pickup_state, hammer_state);
        state = step_rk4(t, dt, &state);
        let Velocity(sample) = pickup_state.velocity;
        sample * gain_out
    };

    let device = alc::Device::open(None).expect("Could not open device");
    let ctx = device.create_context([]).expect("Could not create context");
    ctx.make_current();

    let buffer = al::Buffer::gen();
    let source = al::Source::gen();

    let sample_freq = 44100.0;
    let duration = 20.0;
    let num_samples = (sample_freq * duration) as uint;

    let dt = 1.0 / sample_freq;
    let samples: Vec<i16> = Vec::from_fn(num_samples, |x| {
        let t = (x as f64) / sample_freq;
        (next_sample(t, dt) * (i16::MAX - 1) as f64) as i16
    });

    unsafe { buffer.buffer_data(al::FormatMono16, samples.as_slice(), sample_freq as al::ALsizei) };

    println!("Playing");

    source.queue_buffer(&buffer);
    source.play();
    while source.is_playing() {
        sleep(Duration::milliseconds((duration * 100.0) as i64));
    }

    ctx.destroy();
    device.close().ok().expect("Unable to close device");
}
