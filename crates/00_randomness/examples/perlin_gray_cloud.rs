use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

#[macroquad::main("MyGame")]
async fn main() {
    let image_width = screen_width() as u16;
    let image_height = screen_height() as u16;
    let mut image = Image::gen_image_color(image_width, image_height, WHITE);
    let noise = Perlin::new(0);

    let scale = 1.;

    let mut xoff = 0.0;
    for x in 0..image_width {
        let mut yoff = 0.0;
        for y in 0..image_height {
            let mut pixel_noise = noise.get([xoff, yoff]);
            pixel_noise = (pixel_noise + 1.0) / 2.0 * scale;

            let color = Color {
                r: 1.,
                g: 1.,
                b: 1.,
                a: pixel_noise as f32,
            };

            image.set_pixel(x as u32, y as u32, color);
            yoff += 0.002;
        }
        xoff += 0.002;
    }

    let texture = Texture2D::from_image(&image);

    loop {
        draw_texture(&texture, 0., 0., WHITE);
        next_frame().await
    }
}
