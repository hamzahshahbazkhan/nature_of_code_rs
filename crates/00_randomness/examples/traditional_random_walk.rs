use macroquad::prelude::*;

struct Walker {
    x: f32,
    y: f32,
}
impl Walker {
    fn new() -> Self {
        Self {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0,
        }
    }

    fn show(&self) {
        draw_rectangle(self.x, self.y, 1., 1., GREEN);
    }

    fn step(&mut self) {
        // let choice = rand::gen_range(1, 5);
        // if choice == 1 {
        //     self.x += 1.;
        // } else if choice == 2 {
        //     self.x -= 1.;
        // } else if choice == 3 {
        //     self.y += 1.;
        // } else if choice == 4 {
        //     self.y -= 1.;
        // }

        // This is if you want it to have the tendency to move right.
        // let x_step = rand::gen_range(-3., 3.5);

        let x_step = rand::gen_range(-3, 3);
        let y_step = rand::gen_range(-3, 3);

        self.x += x_step as f32;
        self.y += y_step as f32;
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

        walker.show();
        walker.step();

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
