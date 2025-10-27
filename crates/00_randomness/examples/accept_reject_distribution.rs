use macroquad::prelude::*;

pub fn accept_reject() -> f32 {
    loop {
        let r1 = rand::gen_range(0., 1.);
        let probability = r1; // this is same in this case as the probability and the random number
                              // will be same for the range (0,1);
        let r2 = rand::gen_range(0., 1.);

        if r2 < probability {
            return r1;
        }
    }
}

struct Bars {
    len: u32,
    vec: Vec<f32>,
}

impl Bars {
    fn new(len: u32) -> Self {
        Self {
            len,
            vec: vec![0.; len as usize],
        }
    }

    fn draw(&mut self) {
        let random = (accept_reject() * self.len as f32) as u32;
        self.vec[random as usize] += 1.;

        let w = screen_width() as u32 / self.len;
        let y = screen_height();

        for (i, b) in self.vec.iter().enumerate() {
            let x = i as f32 * w as f32;
            let h = -(*b);
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
