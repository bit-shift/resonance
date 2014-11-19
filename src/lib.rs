#![crate_type = "lib"]
#![crate_name = "resonance"]

#![feature(macro_rules)]

pub mod instrument;
pub mod units;
pub mod runge_kutta;

#[test]
fn rk_test() {
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
