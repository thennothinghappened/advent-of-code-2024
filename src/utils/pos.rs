#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn is_positive(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }

    pub fn from_flat_index(grid_width: usize, index: usize) -> Self {
        let x = index % grid_width;
        let y = (index - x) / grid_width;

        Pos {
            x: x as i32,
            y: y as i32,
        }
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;

    fn sub(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
