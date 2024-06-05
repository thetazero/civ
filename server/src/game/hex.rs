use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Hex<T> {
    pub data: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub struct HexIndex {
    pub row: usize,
    pub col: usize,
}

pub enum HexDirection {
    UpRight,
    Right,
    DownRight,
    DownLeft,
    Left,
    UpLeft,
}

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

// based on: https://www.redblobgames.com/grids/hexagons/
// odd-r offset coordinates
impl<T: Clone> Hex<T> {
    pub fn get(&self, index: HexIndex) -> Option<&T> {
        self.data.get(index.row).and_then(|row| row.get(index.col))
    }

    pub fn new(rows: usize, cols: usize, default: T) -> Self {
        let mut data = vec![];
        for _ in 0..rows {
            let mut row = vec![];
            for _ in 0..cols {
                row.push(default.clone());
            }
            data.push(row);
        }
        Hex { data, rows, cols }
    }

    /// Apply a function to each tile in the hex grid (mutates the grid in place)
    pub fn map(&mut self, map: impl Fn(&T, HexIndex) -> T) -> &mut Hex<T> {
        for (i, row) in self.data.iter_mut().enumerate() {
            for (j, tile) in row.iter_mut().enumerate() {
                let index = HexIndex { row: i, col: j };
                let location = index.to_coords();
                println!("{:?} {:?}", index, location);
                *tile = map(tile, index);
            }
        }
        self
    }

    pub fn for_each(&self, mut action: impl FnMut(&HexIndex, &T)) {
        for (i, row) in self.data.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                let index = HexIndex { row: i, col: j };
                action(&index, tile);
            }
        }
    }

    pub fn collect<U>(&self, mut map: impl FnMut(&HexIndex, &T) -> U) -> Vec<U> {
        let mut result = vec![];
        self.for_each(|index, tile| {
            result.push(map(index, tile));
        });
        result
    }
}

impl HexIndex {
    pub const fn offset(self: &HexIndex, direction: &HexDirection) -> HexIndex {
        if self.row % 2 == 0 {
            // even
            match direction {
                HexDirection::UpRight => HexIndex {
                    row: self.row - 1,
                    col: self.col,
                },
                HexDirection::Right => HexIndex {
                    row: self.row,
                    col: self.col + 1,
                },
                HexDirection::DownRight => HexIndex {
                    row: self.row + 1,
                    col: self.col,
                },
                HexDirection::DownLeft => HexIndex {
                    row: self.row + 1,
                    col: self.col - 1,
                },
                HexDirection::Left => HexIndex {
                    row: self.row,
                    col: self.col - 1,
                },
                HexDirection::UpLeft => HexIndex {
                    row: self.row - 1,
                    col: self.col - 1,
                },
            }
        } else {
            // odd
            match direction {
                HexDirection::UpRight => HexIndex {
                    row: self.row - 1,
                    col: self.col + 1,
                },
                HexDirection::Right => HexIndex {
                    row: self.row,
                    col: self.col + 1,
                },
                HexDirection::DownRight => HexIndex {
                    row: self.row + 1,
                    col: self.col + 1,
                },
                HexDirection::DownLeft => HexIndex {
                    row: self.row + 1,
                    col: self.col,
                },
                HexDirection::Left => HexIndex {
                    row: self.row,
                    col: self.col - 1,
                },
                HexDirection::UpLeft => HexIndex {
                    row: self.row - 1,
                    col: self.col,
                },
            }
        }
    }

    pub fn to_coords(self: &HexIndex) -> Coordinate {
        let size = 1.0;
        let x = size * f64::sqrt(3.0) * (self.col as f64 + 0.5 * (self.row % 2) as f64);
        let y = size * 3.0 / 2.0 * self.row as f64;
        Coordinate { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mut hex = Hex::new(12, 10, 0);
        hex.map(|_, index| {
            println!("{:?}", index);
            1
        });
        assert_eq!(hex.data[0][0], 1);
        assert_eq!(hex.data[11][9], 1);
        assert_eq!(hex.rows, 12);
        assert_eq!(hex.cols, 10);
    }

    #[test]
    fn test_up_right() {
        let index = HexIndex { row: 2, col: 2 };
        assert_eq!(
            index.offset(&HexDirection::UpRight),
            HexIndex { row: 1, col: 2 }
        );

        let index = HexIndex { row: 5, col: 3 };
        assert_eq!(
            index.offset(&HexDirection::UpRight),
            HexIndex { row: 4, col: 4 }
        );
    }
}
