# Resonance

Sound synthesis with [physical modeling][1], simulating a one-dimensional particle system.

The goal is to produce a wide range of realistic sounds that can be played expressively by varying the inputs to the model.

This is my play project for learning Rust. It does not yet produce any sound. If you find this at all interesting or have any comments on the code, do get in touch!

Ben Williamson <benw@pobox.com>

### Description

An "instrument" is a configuration of particles connected by springs. We can visualise an instrument in two or three dimensions, remembering that we only simulate motion in one dimension. A piano string might be modelled as a chain of lightweight particles anchored at both ends, and a "hammer" particle that strikes the string when a note is played.

             o
             v
    O--o--o--o--o--o--O


Each particle has mass, position and velocity.

Springs connect pairs of particles. When the distance between the springs is different from the spring's natural length, the spring applies a restoring force to both particles. The force is equal to the difference from natural length times the spring constant, minus damping.

Springs can also apply damping, absorbing energy by offsetting the restoring force by an amount that is proprotional to the rate of approach of the particles. Without damping an instrument would vibrate forever.

A spring can be one-sided, meaning it only exerts force in one direction. This is useful for modelling particles that collide, such as the hammer that strikes the piano string, a stick striking a drum skin, or a snare coil vibrating against a drum skin.

A particle can be permanently "earthed", preventing it from moving. This stops the whole instrument from drifting away from the origin.

A particle is selected as the "microphone" for the instrument. The velocity of that particle is taken as the amplitude of the output waveform.

The particle system is stepped forward in time using a [Runge-Kutta integrator][2] for stability, producing a single output sample with each step. This means taking 44,100 time steps to produce one second of 44kHz audio.

[1]: http://en.wikipedia.org/wiki/Physical_modelling_synthesis
[2]: http://mathworld.wolfram.com/Runge-KuttaMethod.html