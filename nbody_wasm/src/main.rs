use std::vec;

mod physics;
mod simulation;

fn main() {
    let particle1 = physics::Particle::new([1.0, 0.0, 0.0], [0.0, -4.0, 0.0], 1.0, 1.0);
    let particle2 = physics::Particle::new([-1.0, 0.0, 0.0], [0.0, 4.0, 0.0], 1.0, 1.0);
    let mut particles = vec![particle1, particle2];
    let dt = 0.01;
    for i in 0..100{
        physics::evolve(&mut particles, dt);
        println!("Particle 1 position: {:?}, Particle 2 position: {:?}", 
                    particles[0].position, particles[1].position);
        }
}
