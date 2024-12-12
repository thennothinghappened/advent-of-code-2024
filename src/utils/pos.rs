use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub fn new_from_usize_unchecked(x: usize, y: usize) -> Self {
        Pos {
            x: x as i32,
            y: y as i32,
        }
    }

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

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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
pub trait Index2d<T> {
    type Output;

    fn get_2d(&self, index: Pos) -> Option<&Self::Output>;
    fn get_2d_unchecked(&self, index: Pos) -> &Self::Output;
    fn get_2d_mut(&mut self, index: Pos) -> Option<&mut Self::Output>;
    fn get_2d_mut_unchecked(&mut self, index: Pos) -> &mut Self::Output;
}

impl<T> Index2d<Pos> for Vec<Vec<T>> {
    type Output = T;

    fn get_2d(&self, index: Pos) -> Option<&Self::Output> {
        if !index.is_positive() {
            return None;
        }

        let row = self.get(index.y as usize)?;
        row.get(index.x as usize)
    }

    fn get_2d_unchecked(&self, index: Pos) -> &Self::Output {
        &self[index.y as usize][index.x as usize]
    }

    fn get_2d_mut(&mut self, index: Pos) -> Option<&mut Self::Output> {
        if !index.is_positive() {
            return None;
        }

        let row = self.get_mut(index.y as usize)?;
        row.get_mut(index.x as usize)
    }

    fn get_2d_mut_unchecked(&mut self, index: Pos) -> &mut Self::Output {
        &mut self[index.y as usize][index.x as usize]
    }
}

pub trait FlatIndex<T> {
    type Output: ?Sized;
    fn flat_index(&self, width: usize, index: T) -> &Self::Output;
}

pub trait FlatIndexMut<T>: FlatIndex<T> {
    fn flat_index_mut(&mut self, width: usize, index: T) -> &mut Self::Output;
}

impl<T> FlatIndex<Pos> for [T] {
    type Output = T;

    fn flat_index(&self, width: usize, index: Pos) -> &Self::Output {
        &self[index.x as usize + (index.y as usize) * width]
    }
}

impl<T> FlatIndexMut<Pos> for [T] {
    fn flat_index_mut(&mut self, width: usize, index: Pos) -> &mut Self::Output {
        &mut self[index.x as usize + (index.y as usize) * width]
    }
}
