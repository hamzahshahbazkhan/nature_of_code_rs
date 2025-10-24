use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use rand_distr::{Distribution, Normal};

#[macroquad::main("MyGame")]
async fn main() {
    let mut rng = ::rand::thread_rng();

    let mut r = 1.;
    let mut g = 0.;
    let mut b = 0.;
    let mut a = 0.8;
    let mut color_randomness = 0.1;

    let mut spread = 50.0;
    let mut circle_size = 9.;

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
    clear_background(WHITE);

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
                ui.separator();
                ui.label(None, "circle size");
                widgets::Slider::new(hash!(), 5.0..25.0).ui(ui, &mut circle_size);
                ui.separator();
                ui.label(None, "color values");
                ui.label(None, "r");
                widgets::Slider::new(hash!(), 0.0..1.0).ui(ui, &mut r);
                ui.label(None, "g");
                widgets::Slider::new(hash!(), 0.0..1.0).ui(ui, &mut g);
                ui.label(None, "b");
                widgets::Slider::new(hash!(), 0.0..1.0).ui(ui, &mut b);
                ui.separator();
                ui.label(None, "transparency");
                widgets::Slider::new(hash!(), 0.5..1.0).ui(ui, &mut a);
                ui.separator();
                ui.label(None, "hue range");
                widgets::Slider::new(hash!(), 0.1..1.0).ui(ui, &mut color_randomness);
            });

        let color_range = rand::gen_range(-color_randomness, color_randomness);
        let r_new = r + color_range;
        let g_new = g + color_range;
        let b_new = b + color_range;
        let color = Color {
            r: r_new,
            g: g_new,
            b: b_new,
            a,
        };

        let circle_size_range = rand::gen_range(-1., 1.);

        let new_circle_size = circle_size + circle_size_range;

        let normal = Normal::new(0., spread).unwrap();
        let x_sample = normal.sample(&mut rng);
        let y_sample = normal.sample(&mut rng);

        draw_circle(x + x_sample + 100., y + y_sample, new_circle_size, color);

        set_default_camera();
        clear_background(WHITE);
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

// there is a lot more that could have been added here. But I decided to call it here.
//
