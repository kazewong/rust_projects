struct Particle{
    position: vec3<f32>,
    velocity: vec3<f32>,
    // force: vec3<f32>,
    // mass: f32,
    // radius: f32,
}

const dt = 0.01;
// Create two bind groups, one for computing updates, and one for moving the particles to.

@group(0) @binding(0) var<storage, read> particlesSrc: array<Particle>;
@group(0) @binding(1) var<storage, read_write> particlesDst: array<Particle>;

@compute
@workgroup_size(64)
fn step_main(
    @builtin(global_invocation_id) global_invocation_id: vec3<u32>
){
    let total = arrayLength(&particlesSrc);
    let index = global_invocation_id.x;

    var force = vec3<f32>(0.0, 0.0, 0.0);

    if (index >= total) {
        return;
    }

    for (var i = 0; i < total; i = i + 1) {
        if (i == index) {
            continue;
        }
        let d = particlesSrc[i].position - particlesSrc[index].position;
        let dist = length(d);
        let local_force = normalize(d) / (dist * dist);
        force += local_force;
    }

    particlesDst[index].position = particlesSrc[index].position + particlesSrc[index].velocity * dt;
    particlesDst[index].velocity = particlesSrc[index].velocity + force * dt;

}
