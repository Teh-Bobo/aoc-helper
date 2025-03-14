use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};
use std::str::FromStr;

pub trait PrintBoard {
    fn print_board(&self) -> String;
}

impl<T> PrintBoard for Vec<Vec<T>>
where
    T: Display,
{
    fn print_board(&self) -> String {
        self.iter()
            .map(|row| row.iter().map(|sq| sq.to_string()).collect())
            .reduce(|left, right| format!("{}\n{}", left, right))
            .unwrap_or_default()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn rotate_clockwise(self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }

    pub fn rotate_counterclockwise(self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    }

    /// Based off upper-left being 0,0
    pub fn as_vector(&self) -> Point {
        match self {
            Direction::Left => (-1, 0).into(),
            Direction::Right => (1, 0).into(),
            Direction::Up => (0, -1).into(),
            Direction::Down => (0, 1).into(),
        }
    }
}

pub trait PointOption:
    Add<Output = Self>
    + Sub<Output = Self>
    + Copy
    + Clone
    + Debug
    + Default
    + Eq
    + PartialEq
    + Hash
    + Ord
    + PartialOrd
{
}

impl PointOption for isize {}

impl PointOption for usize {}

impl PointOption for u64 {}
impl PointOption for u32 {}
impl PointOption for u16 {}
impl PointOption for u8 {}

impl PointOption for i64 {}
impl PointOption for i32 {}
impl PointOption for i16 {}
impl PointOption for i8 {}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
pub struct Point<T: PointOption = isize> {
    pub x: T,
    pub y: T,
}

impl<T: PointOption> Point<T> {
    /// Returns true if the test point is located within the box created by upper_left and lower_right inclusive.
    /// Assumes the 0,0 origin is the upper left corner
    pub fn between(&self, upper_left: Point<T>, lower_right: Point<T>) -> bool {
        self.x >= upper_left.x
            && self.x <= lower_right.x
            && self.y >= upper_left.y
            && self.y <= lower_right.y
    }
}

impl<T: PointOption + FromStr> FromStr for Point<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(", ");
        let (start, x) = splits.next().and_then(|x| x.split_once('=')).unwrap();
        assert_eq!(start, "x");

        let (start, y) = splits.next().and_then(|y| y.split_once('=')).unwrap();
        assert_eq!(start, "y");

        Ok(Self {
            x: x.parse().map_err(|_| ())?,
            y: y.parse().map_err(|_| ())?,
        })
    }
}

impl<T: PointOption + Into<isize>> Point<T> {
    pub fn distance(&self, other: &Point<T>) -> usize {
        self.x.into().abs_diff(other.x.into()) + self.y.into().abs_diff(other.y.into())
    }
}

impl<T> Index<Point<usize>> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: Point<usize>) -> &Self::Output {
        &self[index.y][index.x]
    }
}

impl<T> IndexMut<Point<usize>> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: Point<usize>) -> &mut Self::Output {
        &mut self[index.y][index.x]
    }
}

impl<T: PointOption> Add<(T, T)> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: (T, T)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl<T: PointOption> Add<Point<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: PointOption> AddAssign<(T, T)> for Point<T> {
    fn add_assign(&mut self, rhs: (T, T)) {
        self.x = self.x + rhs.0;
        self.y = self.y + rhs.1;
    }
}

impl<T: PointOption> AddAssign<Point<T>> for Point<T> {
    fn add_assign(&mut self, rhs: Point<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: PointOption> Sub<(T, T)> for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: (T, T)) -> Self::Output {
        Self {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
        }
    }
}

impl<T: PointOption> Sub<Point<T>> for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: PointOption> SubAssign<(T, T)> for Point<T> {
    fn sub_assign(&mut self, rhs: (T, T)) {
        self.x = self.x - rhs.0;
        self.y = self.y - rhs.1;
    }
}

impl<T: PointOption> SubAssign<Point<T>> for Point<T> {
    fn sub_assign(&mut self, rhs: Point<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T: PointOption> From<(T, T)> for Point<T> {
    fn from(p: (T, T)) -> Self {
        Self { x: p.0, y: p.1 }
    }
}

impl<T: PointOption> From<Point<T>> for (T, T) {
    fn from(value: Point<T>) -> Self {
        (value.x, value.y)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point3d<T: PointOption = isize> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Point3d {
    pub fn borders(&self) -> [Point3d; 6] {
        [
            (self.x - 1, self.y, self.z).into(),
            (self.x + 1, self.y, self.z).into(),
            (self.x, self.y - 1, self.z).into(),
            (self.x, self.y + 1, self.z).into(),
            (self.x, self.y, self.z - 1).into(),
            (self.x, self.y, self.z + 1).into(),
        ]
    }
}

impl<T: PointOption + FromStr> FromStr for Point3d<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(",");
        let x = splits.next().unwrap().parse().map_err(|_| ())?;
        let y = splits.next().unwrap().parse().map_err(|_| ())?;
        let z = splits.next().unwrap().parse().map_err(|_| ())?;

        Ok(Self { x, y, z })
    }
}

impl<T: PointOption> Add<(T, T, T)> for Point3d<T> {
    type Output = Point3d<T>;

    fn add(self, rhs: (T, T, T)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
            z: self.z + rhs.2,
        }
    }
}

impl<T: PointOption> AddAssign<(T, T, T)> for Point3d<T> {
    fn add_assign(&mut self, rhs: (T, T, T)) {
        self.x = self.x + rhs.0;
        self.y = self.y + rhs.1;
        self.z = self.z + rhs.2;
    }
}

impl<T: PointOption> From<(T, T, T)> for Point3d<T> {
    fn from(p: (T, T, T)) -> Self {
        Self {
            x: p.0,
            y: p.1,
            z: p.2,
        }
    }
}
