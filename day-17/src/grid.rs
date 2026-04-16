use std::{collections::HashMap, fmt};

use crate::point::Point;

#[derive(Default)]
pub struct Grid {
    /// Empty tiles are sand.
    tiles: HashMap<Point, Fill>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Fill {
    Wet,
    Water,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Sand,
    Wet,
    Water,
    Rock,
}

impl Tile {
    pub fn passable(self) -> bool {
        match self {
            Tile::Sand | Tile::Wet => true,
            Tile::Water | Tile::Rock => false,
        }
    }
}

impl Grid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, p: Point) -> Tile {
        match self.tiles.get(&p) {
            None => Tile::Sand,
            Some(tile) => match tile {
                Fill::Wet => Tile::Wet,
                Fill::Water => Tile::Water,
                Fill::Rock => Tile::Rock,
            },
        }
    }

    pub fn set(&mut self, p: Point, tile: Tile) {
        let fill = match tile {
            Tile::Sand => {
                self.tiles.remove(&p);
                return;
            }
            Tile::Wet => Fill::Wet,
            Tile::Water => Fill::Water,
            Tile::Rock => Fill::Rock,
        };
        self.tiles.insert(p, fill);
    }

    pub fn count_wet(&self, min_y: i64, max_y: i64) -> usize {
        self.count(Fill::Wet, min_y, max_y)
    }

    pub fn count_water(&self, min_y: i64, max_y: i64) -> usize {
        self.count(Fill::Water, min_y, max_y)
    }

    fn count(&self, tile: Fill, min_y: i64, max_y: i64) -> usize {
        self.tiles
            .iter()
            .filter(|&(&p, &t)| (min_y..=max_y).contains(&p.y) && t == tile)
            .count()
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tiles.is_empty() {
            return write!(f, "(empty)");
        }

        let min_x = self.tiles.keys().map(|p| p.x).min().unwrap();
        let max_x = self.tiles.keys().map(|p| p.x).max().unwrap();
        let min_y = self.tiles.keys().map(|p| p.y).min().unwrap();
        let max_y = self.tiles.keys().map(|p| p.y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c = match self.get(Point { x, y }) {
                    Tile::Sand => '.',
                    Tile::Wet => '|',
                    Tile::Water => '~',
                    Tile::Rock => '#',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
