use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Formatter;
use std::iter;
use std::ops::{Add, Mul, Sub};

pub type Direction = XY;
pub const DIR_UP: Direction = XY { x: 0, y: -1 };
pub const DIR_DOWN: Direction = XY { x: 0, y: 1 };
pub const DIR_LEFT: Direction = XY { x: -1, y: 0 };
pub const DIR_RIGHT: Direction = XY { x: 1, y: 0 };

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct XY {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone)]
pub struct DenseGrid<T> {
    width: i64,
    height: i64,
    cells: Vec<T>,
}

impl<T: Clone> DenseGrid<T> {
    pub fn from_rows(rows: Vec<Vec<T>>) -> DenseGrid<T> {
        let width = rows[0].len() as i64;
        let height = rows.len() as i64;
        let cells = rows.into_iter().flat_map(|row| row.into_iter()).collect();
        DenseGrid {
            width,
            height,
            cells,
        }
    }

    pub fn from_iter(width: usize, elements: impl Iterator<Item = T>) -> DenseGrid<T> {
        let cells: Vec<_> = elements.collect();
        if cells.len() % width != 0 {
            panic!(
                "grid is not a square, width={width}, length={}",
                cells.len()
            );
        }
        let height = cells.len() / width;
        DenseGrid {
            width: width as i64,
            height: height as i64,
            cells,
        }
    }

    pub fn at(&self, (x, y): (i64, i64)) -> Option<&T> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            Some(&self.cells[(x + y * self.width) as usize])
        }
    }

    pub fn try_set_at(&mut self, (x, y): (i64, i64), val: T) -> Option<()> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            self.cells[(x + y * self.width) as usize] = val;
            Some(())
        }
    }

    pub fn set_at(&mut self, pos: (i64, i64), val: T) {
        self.try_set_at(pos, val).unwrap()
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        (0..self.height)
            .map(|y| &self.cells[(y * self.width) as usize..((y + 1) * self.width) as usize])
    }

    pub fn items(&self) -> impl Iterator<Item = (XY, &T)> {
        self.cells.iter().enumerate().map(|(i, value)| {
            (
                XY {
                    x: (i as i64) % self.width,
                    y: (i as i64) / self.width,
                },
                value,
            )
        })
    }

    pub fn width(&self) -> i64 {
        self.width
    }

    pub fn height(&self) -> i64 {
        self.height
    }
}

impl<T: Eq> DenseGrid<T> {
    pub fn find<'a, 'b>(&'a self, value: &'b T) -> impl Iterator<Item = XY> + use<'a, 'b, T> {
        self.cells
            .iter()
            .enumerate()
            .filter(move |(_, item)| item == &value)
            .map(|(i, _)| XY {
                x: (i as i64) % self.width,
                y: (i as i64) / self.width,
            })
    }
}

impl XY {
    pub fn as_tuple(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    pub fn rotate_90_cw(&self) -> XY {
        XY {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn cardinal_neighbours(&self) -> impl Iterator<Item = XY> + Clone {
        iter::once(*self + DIR_UP)
            .chain(iter::once(*self + DIR_DOWN))
            .chain(iter::once(*self + DIR_LEFT))
            .chain(iter::once(*self + DIR_RIGHT))
    }

    pub fn corner_neighbours(&self) -> impl Iterator<Item = XY> + Clone {
        iter::once(*self + DIR_UP + DIR_LEFT)
            .chain(iter::once(*self + DIR_UP + DIR_RIGHT))
            .chain(iter::once(*self + DIR_DOWN + DIR_LEFT))
            .chain(iter::once(*self + DIR_DOWN + DIR_RIGHT))
    }

    pub fn all_neighbours(&self) -> impl Iterator<Item = XY> + Clone {
        self.cardinal_neighbours().chain(self.corner_neighbours())
    }

    pub fn length_sq(&self) -> i64 {
        self.x * self.x + self.y * self.y
    }

    pub fn taxicab_length(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

impl Add for XY {
    type Output = XY;
    fn add(self, rhs: XY) -> XY {
        XY {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for XY {
    type Output = XY;
    fn sub(self, rhs: XY) -> XY {
        XY {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i64> for XY {
    type Output = XY;
    fn mul(self, rhs: i64) -> Self::Output {
        XY {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<(i64, i64)> for XY {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + Copy + Clone> std::fmt::Display for DenseGrid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rows().for_each(|row| {
            f.write_str(row.iter().join("").as_str()).unwrap();
            f.write_str("\n").unwrap();
        });
        Ok(())
    }
}

pub fn area_outline(area: &HashSet<XY>) -> HashSet<XY> {
    area.iter()
        .flat_map(XY::all_neighbours)
        .filter(|p| !area.contains(p))
        .collect()
}
