use macroquad::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};

// const COLORS: Color = Color::new {
//     r: 0.,
//     g: 1.,
//     b: 0.,
//     a: 0.,
// };

struct Walker {
    x: f32,
    y: f32,
    tx: f32,
    ty: f32,
}

impl Walker {
    fn new() -> Self {
        Self {
            x: screen_width() / 2.,
            y: screen_height() / 2.,
            tx: 0.,
            ty: 100000.,
        }
    }
    fn show(&self) {
        draw_circle(self.x, self.y, 18., GREEN);
        draw_circle_lines(self.x, self.y, 18., 1., BLACK);
    }
    fn step(&mut self, noise: &Perlin) {
        // let perlin = Perlin::new(1);
        let nx = noise.get([self.tx as f64]) as f32;
        let ny = noise.get([self.ty as f64]) as f32;

        self.x = ((nx as f32 + 1.0) / 2.0) * screen_width();
        self.y = ((ny as f32 + 1.0) / 2.0) * screen_height();
        self.tx += 0.01;
        self.ty += 0.01;
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut walker = Walker::new();
    let perlin = Perlin::new(0);

    let render_target = render_target(screen_width() as u32, screen_height() as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let camera = Camera2D {
        zoom: vec2(2.0 / screen_width(), -2.0 / screen_height()),
        target: vec2(screen_width() / 2.0, screen_height() / 2.0),
        render_target: Some(render_target.clone()),
        ..Default::default()
    };

    set_camera(&camera);
    clear_background(BLACK);
    loop {
        set_camera(&camera);
        walker.show();
        walker.step(&perlin);

        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );

        next_frame().await
    }
}
