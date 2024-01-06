pub struct Particle{
    pub position: [f32; 3],
    velocity: [f32; 3],
    force: [f32; 3],
    mass: f32,
    radius: f32,
}

impl Particle{
    pub fn new(position: [f32; 3], velocity: [f32; 3], mass: f32, radius: f32) -> Self{
        Self{
            position,
            velocity,
            force: [0.0, 0.0, 0.0],
            mass,
            radius,
        }
    }

    fn compute_force(&self, other_particle: &Particle) -> [f32; 3]{
        let distance: [f32; 3] = [
            self.position[0] - other_particle.position[0],
            self.position[1] - other_particle.position[1],
            self.position[2] - other_particle.position[2],
        ];
        let distance_norm = (distance[0] * distance[0] + distance[1] * distance[1] + distance[2] * distance[2]).sqrt();
        let mut result = [0.0, 0.0, 0.0]; 
        for i in 0..3{
            result[i] += distance[i] / distance_norm * self.mass * other_particle.mass;
        }
        result
    }
}

fn compute_forces(particles: &mut Vec<Particle>) -> (){
    for i in 0..particles.len(){
        for j in i+1..particles.len(){
            let force = particles[i].compute_force(&particles[j]);
            for k in 0..3{
                particles[i].force[k] += force[k];
                particles[j].force[k] -= force[k];
            }
        }
    }
}

pub fn evolve(particles: &mut Vec<Particle>, dt: f32) -> (){
    // Compute forces
    compute_forces(particles);
    for i in 0..particles.len(){
        for j in 0..3{
            particles[i].velocity[j] += particles[i].force[j] * dt / particles[i].mass;
            particles[i].position[j] += particles[i].velocity[j] * dt;
        }
    }
}