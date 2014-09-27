
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

// Units for physical dimensions
pub struct Mass(pub f64);       // kg

#[deriving(Clone)]
pub struct Length(pub f64);     // m
pub struct Time(pub f64);       // s

#[deriving(Clone)]
pub struct Velocity(f64);       // m/s
pub struct Force(pub f64);      // N
pub struct Stiffness(pub f64);  // N/m
pub struct Damping(pub f64);    // N/m/s

addable_newtype!(Length)
scalable_newtype!(Length)
addable_newtype!(Velocity)
scalable_newtype!(Velocity)

// Opaque handles on components of an Instrument.
// Privately these are just indexes into Instrument's vectors.
pub struct Particle(uint);
pub struct Spring(uint);

// Specification of a particle
pub struct ParticleSpec {
    pub mass: Mass,
    pub position: Length
}

// Specification of a spring
pub struct SpringSpec {
    pub p0: Particle,
    pub p1: Particle,
    pub natural_length: Length,
    pub stiffness: Stiffness,
    pub damping: Damping,
    pub one_sided: bool
}

pub struct Instrument {
    particles: Vec<ParticleSpec>,
    springs: Vec<SpringSpec>,
    earths: Vec<Particle>
}

impl Instrument {
    pub fn new() -> Instrument {
        Instrument{
            particles: Vec::new(),
            springs: Vec::new(),
            earths: Vec::new()
        }
    }

    pub fn add_particle(&mut self, mass: Mass, position: Length) -> Particle {
        self.particles.push(ParticleSpec{ mass: mass, position: position });
        Particle(self.particles.len() - 1)
    }

    pub fn add_spring(&mut self,
                        p0: Particle,
                        p1: Particle,
                        natural_length: Length,
                        stiffness: Stiffness,
                        damping: Damping,
                        one_sided: bool) -> Spring {
        self.springs.push(SpringSpec{
            p0: p0,
            p1: p1,
            natural_length: natural_length,
            stiffness: stiffness,
            damping: damping,
            one_sided: one_sided
        });
        Spring(self.springs.len() - 1)
    }

    pub fn earth(&mut self, p: Particle) {
        self.earths.push(p);
    }
}

#[deriving(Clone)]
struct ParticleState {
    position: Length,
    velocity: Velocity,
}

impl ParticleState {
    fn new() -> ParticleState {
        ParticleState { position: Length(0.0), velocity: Velocity(0.0) }
    }
}

impl Add<ParticleState, ParticleState> for ParticleState {
    fn add(&self, rhs: &ParticleState) -> ParticleState {
        ParticleState {
            position: self.position + rhs.position,
            velocity: self.velocity + rhs.velocity
        }
    }
}

impl Mul<f64, ParticleState> for ParticleState {
    fn mul(&self, &rhs: &f64) -> ParticleState {
        ParticleState {
            position: self.position * rhs,
            velocity: self.velocity * rhs
        }
    }
}

#[deriving(Clone)]
struct InstrumentState<'a> {
    particle_states: Vec<ParticleState>,
    instrument: &'a Instrument
}

impl<'a> Add<InstrumentState<'a>, InstrumentState<'a>> for InstrumentState<'a> {
    fn add(&self, rhs: &InstrumentState) -> InstrumentState<'a> {
        assert_eq!(self.particle_states.len(), rhs.particle_states.len());
        InstrumentState {
            particle_states: self.particle_states.iter().enumerate().map(|(index, &left)| {
                left + rhs.particle_states[index]
            }).collect(),
            instrument: self.instrument
        }
    }
}

impl<'a> Mul<f64, InstrumentState<'a>> for InstrumentState<'a> {
    fn mul(&self, &rhs: &f64) -> InstrumentState<'a> {
        InstrumentState {
            particle_states: self.particle_states.iter().map(|&left| {
                left * rhs
            }).collect(),
            instrument: self.instrument
        }
    }
}

impl<'a> ::runge_kutta::State for InstrumentState<'a> {
    fn f(&self, _: f64) -> InstrumentState<'a> {

        let mut result = InstrumentState {
            particle_states: Vec::from_elem(self.instrument.particles.len(), ParticleState::new()),
            instrument: self.instrument
        };



        result
    }
}
