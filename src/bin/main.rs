use raytracer::{camera::Camera, output, world::World};

fn main() {
    let camera = Camera::default();
    let world = World::new_random();

    output::write(&camera, &world);
}
