mod ray_tracer;

use ray_tracer::RayTracer;
use ray_tracer::Scene;

fn main() {
    let mut ray_tracer = RayTracer::new();
    ray_tracer.set_scene(Scene::EX1);
    ray_tracer.render();
}
