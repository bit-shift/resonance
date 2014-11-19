
macro_rules! addable_newtype(
    ($T:ident) => (
        impl Add<$T, $T> for $T {
            fn add(&self, rhs: &$T) -> $T {
                let ($T(self_val), $T(rhs_val)) = (*self, *rhs);
                $T(self_val + rhs_val)
            }
        }
    )
)

macro_rules! subable_newtype(
    ($T:ident) => (
        impl Sub<$T, $T> for $T {
            fn sub(&self, rhs: &$T) -> $T {
                let ($T(self_val), $T(rhs_val)) = (*self, *rhs);
                $T(self_val - rhs_val)
            }
        }
    )
)

macro_rules! scalable_newtype(
    ($T:ident) => (
        impl Mul<f64, $T> for $T {
            fn mul(&self, &rhs: &f64) -> $T {
                let $T(self_val) = *self;
                $T(self_val * rhs)
            }
        }
    )
)

macro_rules! mul_newtypes(
    ($A:ident * $B:ident -> $C:ident) => (
        impl Mul<$B, $C> for $A {
            fn mul(&self, rhs: &$B) -> $C {
                let $A(self_val) = *self;
                let $B(rhs_val) = *rhs;
                $C(self_val * rhs_val)
            }
        }
    )
)
// Units for physical dimensions

#[deriving(Clone, Show)]
pub struct Mass(pub f64);       // kg

#[deriving(Clone, Show)]
pub struct Length(pub f64);     // m

#[deriving(Clone, Show)]
pub struct Time(pub f64);       // s

#[deriving(Clone, Show)]
pub struct Velocity(pub f64);   // m/s

#[deriving(Clone, Show)]
pub struct Accel(pub f64);      // m/s/s

#[deriving(Clone, Show)]
pub struct Force(pub f64);      // N

#[deriving(Clone, Show)]
pub struct Stiffness(pub f64);  // N/m

#[deriving(Clone, Show)]
pub struct Damping(pub f64);    // N/m/s


addable_newtype!(Length)
subable_newtype!(Length)
scalable_newtype!(Length)
addable_newtype!(Velocity)
subable_newtype!(Velocity)
scalable_newtype!(Velocity)
addable_newtype!(Force)
subable_newtype!(Force)

mul_newtypes!(Stiffness * Length -> Force)
mul_newtypes!(Damping * Velocity -> Force)


impl Div<Mass, Accel> for Force {
    fn div(&self, rhs: &Mass) -> Accel {
        let (Force(self_val), Mass(rhs_val)) = (*self, *rhs);
        Accel(self_val / rhs_val)
    }
}
