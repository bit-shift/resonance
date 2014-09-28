#![feature(macro_rules)]

use runge_kutta::{step_rk2, step_rk4};

mod runge_kutta;

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

fn simple_rk_test {
    // This is currently just a test of the RK integrators:
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
