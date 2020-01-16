use crate::canvas::Canvas;

use stdweb::console;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub tx: f64,
    pub ty: f64,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Player {
        Player { x, y, tx: x, ty: y }
    }
    pub fn update(&mut self) {
        let mag = ((self.tx - self.x) * (self.tx - self.x)
            + (self.ty - self.y) * (self.ty - self.y))
            .sqrt();

        // let mut v = [0.0, 0.0];
        let v = if mag != 0.0 {
            ((self.tx - self.x) / mag, (self.ty - self.y) / mag)
        } else {
            (0.0, 0.0)
        };

        self.x += v.0;
        self.y += v.1;
    }

    pub fn set_target(&mut self, tx: i32, ty: i32) {
        self.tx = tx.into();
        self.ty = ty.into();
    }

    pub fn draw(&self, canvas: &Canvas) {
        canvas.draw(self.x, self.y, "orange");
    }
}
