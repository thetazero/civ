use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Hex<T> {
    pub data: Vec<Vec<T>>,
}

#[derive(Debug, PartialEq)]
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
    pub fn new(rows: usize, cols: usize, default: T) -> Self {
        let mut data = vec![];
        for _ in 0..rows {
            let mut row = vec![];
            for _ in 0..cols {
                row.push(default.clone());
            }
            data.push(row);
        }
        Hex { data }
    }

    pub fn map_mut(
        &mut self,
        map: impl Fn(&T, HexIndex) -> T,
    ) -> &mut Hex<T> {
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
        let mut hex = Hex::new(10, 10, 0);
        hex.map_mut(|tile, index| {
            println!("{:?}", index);
            1
        });
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
