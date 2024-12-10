use std::{ops::Index, slice::SliceIndex};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn is_positive(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }

    pub fn is_in_rect(&self, top_left: &Pos, bottom_right: &Pos) -> bool {
        self.x >= top_left.x
            && self.y >= top_left.y
            && self.x <= bottom_right.x
            && self.y <= bottom_right.y
    }

    pub fn is_valid_grid_index(&self, grid_width: usize, grid_height: usize) -> bool {
        self.is_positive() && (self.x as usize) < grid_width && (self.y as usize) < grid_height
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

impl std::ops::Mul for Pos {
    type Output = Pos;

    fn mul(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl From<i32> for Pos {
    fn from(value: i32) -> Self {
        Pos { x: value, y: value }
    }
}

/// Rust did not want to cooperate with me implementing `Index` to use with `Vec`.
trait Index2<T> {
    type Output;

    fn index(&self, index: Pos) -> &Self::Output;
}

impl<T> Index2<Pos> for [&Vec<T>] {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[index.y as usize][index.x as usize]
    }
}
