use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use rand_distr::{Distribution, Normal};

#[macroquad::main("MyGame")]
async fn main() {
    let mut rng = ::rand::thread_rng();

    let r = 1.;
    let g = 0.;
    let b = 0.;
    let a = 0.5;

    let mut spread = 50.0;
    let mut circle_size = 5.;
    let mut COLOR = Color { r, g, b, a };

    let x = screen_width() / 2.;
    let y = screen_height() / 2.;

    let render_target = render_target(screen_width() as u32, screen_height() as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    let camera = Camera2D {
        target: vec2(screen_width() / 2., screen_height() / 2.),
        zoom: vec2(2. / screen_width(), -2. / screen_height()),
        render_target: Some(render_target.clone()),
        ..Default::default()
    };

    set_camera(&camera);
    clear_background(BLACK);

    let window_position = vec2(0., 0.);
    let window_size = vec2(200., screen_height());

    loop {
        set_camera(&camera);

        widgets::Window::new(hash!(), window_position, window_size)
            .movable(false)
            .titlebar(false)
            .ui(&mut root_ui(), |ui| {
                ui.label(None, "scatter");
                widgets::Slider::new(hash!(), 20.0..200.0).ui(ui, &mut spread);
            });

        let normal = Normal::new(0., spread).unwrap();
        let x_sample = normal.sample(&mut rng);
        let y_sample = normal.sample(&mut rng);

        draw_circle(x + x_sample, y + y_sample, circle_size, COLOR);

        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
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
