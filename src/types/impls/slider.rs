use crate::types::defs::*;

impl Slider {
    pub fn new(x: isize, y: isize, slide: bool, move_fn: fn(&Board, Tile) -> bool) -> Self {
        Self {
            x,
            y,
            slide,
            move_fn,
        }
    }
    pub fn self_xy(&self, x: isize, y: isize) -> Self {
        Slider {
            x,
            y,
            slide: self.slide,
            move_fn: self.move_fn,
        }
    }
    pub fn siblings(&self) -> Vec<Slider> {
        let x = self.x;
        let y = self.y;
        if x == y {
            vec![
                self.self_xy(x, y),
                self.self_xy(x, -y),
                self.self_xy(-x, y),
                self.self_xy(-x, -y),
            ]
        } else if x == 0 {
            vec![
                self.self_xy(x, y),
                self.self_xy(x, -y),
                self.self_xy(y, x),
                self.self_xy(-y, x),
            ]
        } else if y == 0 {
            vec![
                self.self_xy(x, y),
                self.self_xy(-x, y),
                self.self_xy(y, x),
                self.self_xy(y, -x),
            ]
        } else {
            vec![
                self.self_xy(x, y),
                self.self_xy(x, -y),
                self.self_xy(-x, y),
                self.self_xy(-x, -y),
                self.self_xy(y, x),
                self.self_xy(y, -x),
                self.self_xy(-y, x),
                self.self_xy(-y, -x),
            ]
        }
    }
}
