use ::rand::rngs::ThreadRng;
use macroquad::prelude::*;
use rand_distr::{Distribution, Normal};

struct Walker {
    x: f32,
    y: f32,
}

impl Walker {
    fn new() -> Self {
        Self {
            x: screen_width() / 2.,
            y: screen_height() / 2.,
        }
    }
    fn show(&self) {
        draw_rectangle(self.x, self.y, 1., 1., PURPLE);
    }

    fn walk(&mut self, mut rng: &mut ThreadRng) {
        let normal = Normal::new(0.0, 3.0).unwrap();
        let sample_x = normal.sample(&mut rng);
        let sample_y = normal.sample(&mut rng);

        self.x += sample_x;
        self.y += sample_y;
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut rng = ::rand::rng();
    let mut walker = Walker::new();

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
        // clear_background(BLACK);

        walker.show();
        walker.walk(&mut rng);

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
