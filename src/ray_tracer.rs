pub enum Scene {
    EX1,
}

pub struct RayTracer {
    scene: Scene,
}

impl RayTracer {
    pub fn new() -> Self {
        Self { scene: Scene::EX1 }
    }
    pub fn render(&mut self) {
        let mut imgbuf = self.setup();
        self.trace_rays(&mut imgbuf);
        self.save_image(&imgbuf);
    }

    pub fn set_scene(&mut self, scene: Scene) {
        self.scene = scene;
    }

    fn setup(&self) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let (width, height) = self.get_scene_resolution();
        let imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            image::ImageBuffer::new(width, height);
        imgbuf
    }

    fn trace_rays(&self, imgbuf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let (r, g, b) = self.calculate_color(x, y);
            *pixel = image::Rgb([r, g, b]);
        }
    }

    fn calculate_color(&self, x: u32, y: u32) -> (u8, u8, u8) {
        match self.scene {
            Scene::EX1 => self.scene_1(x, y),
        }
    }

    fn save_image(&self, imgbuf: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
        let _ = imgbuf.save("raytraced.png").unwrap();
    }

    fn get_scene_resolution(&self) -> (u32, u32) {
        match self.scene {
            Scene::EX1 => (256, 256),
        }
    }

    fn scene_1(&self, _x: u32, _y: u32) -> (u8, u8, u8) {
        (128 as u8, 128 as u8, 128 as u8)
    }
}
