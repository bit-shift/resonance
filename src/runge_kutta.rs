// http://mathworld.wolfram.com/Runge-KuttaMethod.html

pub trait State: Add<Self, Self> + Mul<f64, Self> {
    fn f(&self, t: f64) -> Self;
}

#[allow(dead_code)]
pub fn step_rk2<Y>(t: f64, dt: f64, y: &Y) -> Y
where Y: State {
    let k_1 = y.f(t) * dt;
    let k_2 = (*y + k_1 * 0.5).f(t + 0.5 * dt) * dt;
    *y + k_2
}

pub fn step_rk4<Y>(t: f64, dt: f64, y: &Y) -> Y
where Y: State {
    let k_1 = y.f(t) * dt;
    let k_2 = (*y + k_1 * 0.5).f(t + 0.5 * dt) * dt;
    let k_3 = (*y + k_2 * 0.5).f(t + 0.5 * dt) * dt;
    let k_4 = (*y + k_3).f(t + dt) * dt;
    *y + k_1 * (1.0 / 6.0) + k_2 * (1.0 / 3.0) + k_3 * (1.0 / 3.0) + k_4 * (1.0 / 6.0)
}
