#![feature(macro_rules)]

use runge_kutta::{step_rk2, step_rk4};
use instrument::{
    Mass,
    Length,
    Stiffness,
    Damping,
    Instrument
};

mod runge_kutta;
mod instrument;

#[deriving(Show)]
struct State {
    p: f64,
    v: f64
}

impl Add<State, State> for State {
    fn add(&self, rhs: &State) -> State {
        State { p: self.p + rhs.p, v: self.v + rhs.v }
    }
}

impl Mul<f64, State> for State {
    fn mul(&self, rhs: &f64) -> State {
        State { p: self.p * (*rhs), v: self.v * (*rhs) }
    }
}

impl runge_kutta::State for State {
    fn f(&self, _: f64) -> State {
        State { p: self.v, v: -0.1 * self.p }
    }
}

fn main() {
    let mut instrument = Instrument::new();
    let p0 = instrument.add_particle(Mass(1.0), Length(0.0));
    let p1 = instrument.add_particle(Mass(1.0), Length(0.0));
    let spring = instrument.add_spring(
                    p0, p1, Length(0.0), Stiffness(1.0), Damping(1e-3), false);

    let mut t = 0f64;
    let mut y2 = State { p: 1.0f64, v: 0.0f64 };
    let mut y4 = y2;
    let dt = 0.5f64;
    for _ in range(0i, 1000) {
        println!("{:0.1f}\t{:+0.6f}\t{:+0.6f}", t, y2.p, y4.p);
        y2 = step_rk2(t, dt, &y2);
        y4 = step_rk4(t, dt, &y4);
        t += dt;
    }
}
