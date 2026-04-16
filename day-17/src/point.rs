use std::ops::Add;

/// y grows down.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub const UP: Self = Self { x: 0, y: -1 };
    pub const DOWN: Self = Self { x: 0, y: 1 };
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };

    pub fn up(self) -> Self {
        self + Self::UP
    }

    pub fn down(self) -> Self {
        self + Self::DOWN
    }

    pub fn left(self) -> Self {
        self + Self::LEFT
    }

    pub fn right(self) -> Self {
        self + Self::RIGHT
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
