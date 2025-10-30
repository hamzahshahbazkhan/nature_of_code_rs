use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

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
            ty: 10000.,
        }
    }
    fn show(&self) {
        draw_circle(self.x, self.y, 1., PURPLE);
        // draw_circle_lines(self.x, self.y, 14., 1., BLACK);
    }
    fn step(&mut self, noise: &Perlin) {
        // useing 2D noise as 1D noise result was too deterministic
        let x_step = noise.get([self.tx as f64, self.tx as f64]);
        let y_step = noise.get([self.ty as f64, self.ty as f64]);

        self.tx += 0.01;
        self.ty += 0.01;

        // println!("x: {x_step}");
        // println!("y: {y_step}");

        self.x += x_step as f32;
        self.y += y_step as f32;
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut walker = Walker::new();
    let noise = Perlin::new(1);

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
        walker.step(&noise);

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
