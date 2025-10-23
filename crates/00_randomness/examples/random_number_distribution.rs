use macroquad::prelude::*;

struct Bars {
    length: u32,
    vector: Vec<u32>,
}

impl Bars {
    fn new(length: u32) -> Self {
        let vec = vec![0; length as usize];
        Self {
            length,
            vector: vec,
        }
    }

    fn draw(&mut self) {
        let random = rand::gen_range(0, self.length);
        self.vector[random as usize] += 1;

        let w = screen_width() as u32 / self.length;

        for (i, b) in self.vector.iter().enumerate() {
            let x = i as f32 * w as f32;
            let y = screen_height();
            let h = -(*b as f32);
            draw_rectangle(x, y, w as f32, h, ORANGE);
            draw_rectangle_lines(x, y, w as f32, h, 2., BLACK);
        }
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut bars = Bars::new(20);
    loop {
        clear_background(BLACK);

        bars.draw();

        next_frame().await
    }
}
