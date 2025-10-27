use macroquad::prelude::*;

pub fn accept_reject() -> f32 {
    loop {
        let r1 = rand::gen_range(0., 1.);
        let probability = r1;
        let r2 = rand::gen_range(0., 1.);

        if r2 < probability {
            return r1;
        }
    }
}

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
        draw_rectangle(self.x, self.y, 1., 1., GREEN);
    }

    fn step(&mut self) {
        let step = 5.;
        let mut x_step = accept_reject() * step;
        let mut y_step = accept_reject() * step;

        let bool = [true, false];

        if bool[rand::gen_range(0, 2)] {
            x_step *= -1.;
        }
        if bool[rand::gen_range(0, 2)] {
            y_step *= -1.;
        }

        self.x += x_step;
        self.y += y_step;
    }
}

#[macroquad::main("MyGame")]
async fn main() {
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

        walker.step();
        walker.show();

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
