use crate::math::{Point, Vector, DIM};
use na::{self, DVector, RealField};

pub struct Fluid<N: RealField> {
    pub positions: Vec<Point<N>>,
    pub velocities: Vec<Vector<N>>,
    pub volumes: DVector<N>,
    pub density0: N,
    pub viscosity: N,
    assembly_id: usize,
}

impl<N: RealField> Fluid<N> {
    pub fn new(
        particle_positions: Vec<Point<N>>,
        particle_radius: N,
        density0: N,
        viscosity: N,
    ) -> Self
    {
        let num_particles = particle_positions.len();
        let velocities = std::iter::repeat(Vector::zeros())
            .take(num_particles)
            .collect();
        #[cfg(feature = "dim2")]
        let particle_volume = particle_radius * particle_radius * N::pi();
        #[cfg(feature = "dim3")]
        let particle_volume =
            particle_radius * particle_radius * particle_radius * N::pi() * na::convert(4.0 / 3.0);

        Self {
            positions: particle_positions,
            velocities,
            volumes: DVector::repeat(num_particles, particle_volume),
            density0,
            viscosity,
            assembly_id: 0,
        }
    }

    pub fn assembly_id(&self) -> usize {
        self.assembly_id
    }

    pub fn set_assembly_id(&mut self, id: usize) {
        self.assembly_id = id
    }

    pub fn num_particles(&self) -> usize {
        self.positions.len()
    }
}