mod ray_tracer;

use ray_tracer::RayTracer;

fn main() {
    let ray_tracer = RayTracer::new();
    ray_tracer.render();
}
