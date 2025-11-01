use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
use std::time::Duration;

fn draw_wireframe(
    width: f32,
    height: f32,
    segment_x: u16,
    segment_y: u16,
    offset: f64,
    terrain: &mut Vec<Vec<f64>>,
    noise: &Perlin,
) {
    let len_x = width * 2. / segment_x as f32;
    let len_y = height * 2. / segment_y as f32;

    let mut yoff = 0.0;
    let mut xoff = offset;

    for i in 0..segment_y {
        // let mut yoff = 0.0;
        for j in 0..segment_x {
            // let mut xoff = 0.0;
            terrain[i as usize][j as usize] = noise.get([xoff, yoff]);
            xoff += 0.1;
        }
        yoff += 0.1;
    }

    let mut start_x = -width;
    let mut start_y = height;
    // let start_z = 0.0;

    for i in 0..segment_y - 1 {
        for j in 0..segment_x - 1 {
            let point_1 = vec3(start_x, terrain[i as usize][j as usize] as f32, start_y);
            let point_2 = vec3(
                start_x + len_x,
                terrain[i as usize][(j + 1) as usize] as f32,
                start_y,
            );
            let point_3 = vec3(
                start_x,
                terrain[(i + 1) as usize][j as usize] as f32,
                start_y - len_y,
            );

            if j == segment_x && i != segment_y {
                draw_line_3d(point_3, point_1, PINK);
            } else if i == segment_y && j != segment_x {
                draw_line_3d(point_1, point_2, PINK);
            } else if i != segment_y && j != segment_x {
                draw_line_3d(point_1, point_2, PINK);
                draw_line_3d(point_2, point_3, PINK);
                draw_line_3d(point_3, point_1, PINK);
            }

            start_x += len_x;
        }
        start_x = -width;
        start_y -= len_y;
    }
}

#[macroquad::main("Plane Wireframe")]
async fn main() {
    let mut terrain: Vec<Vec<f64>> = vec![vec![0.; X as usize]; Y as usize];
    let mut noise = Perlin::new(1);
    let mut offset: f64 = 0.0;

    const X: u16 = 70;
    const Y: u16 = 70;

    loop {
        clear_background(DARKPURPLE);

        set_camera(&Camera3D {
            position: vec3(12., 6., 0.), // Further away
            up: vec3(0., 1., 0.),        // Correct up direction
            target: vec3(0., 0., 0.),
            fovy: 45.0,
            ..Default::default()
        });

        draw_wireframe(12.0, 18.0, X, Y, offset, &mut terrain, &mut noise);
        offset -= 0.1;

        set_default_camera();

        next_frame().await;

        std::thread::sleep(Duration::from_millis(30));
    }
}
