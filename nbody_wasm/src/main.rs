use nbody_wasm;

fn main() {
    pollster::block_on(nbody_wasm::run());
}