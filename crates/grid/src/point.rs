use crate::{Direction, Grid, GridCell};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub const fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn on<'a, T: std::ops::Index<usize>>(&self, grid: &'a Grid<T>) -> Option<GridCell<'a, T>> {
        grid.cell_at_point(self)
    }

    pub fn up(&self) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            Some(Self::new(self.x, self.y - 1))
        }
    }

    pub fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    pub fn left(&self) -> Option<Self> {
        if self.x == 0 {
            None
        } else {
            Some(Self::new(self.x - 1, self.y))
        }
    }

    pub fn right(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    pub fn go(&self, direction: &Direction) -> Option<Self> {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => Some(self.down()),
            Direction::Left => self.left(),
            Direction::Right => Some(self.right()),
        }
    }

    pub fn manhattan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl std::ops::Add<(isize, isize)> for Point {
    type Output = Option<Point>;

    fn add(self, (dx, dy): (isize, isize)) -> Self::Output {
        let x: isize = self.x.try_into().ok()?;
        let y: isize = self.y.try_into().ok()?;

        Some(Self::new(
            (x + dx).try_into().ok()?,
            (y + dy).try_into().ok()?,
        ))
    }
}

impl std::ops::Add<Direction> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: Direction) -> Self::Output {
        self + match rhs {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl std::ops::Sub<(isize, isize)> for Point {
    type Output = Option<Point>;

    fn sub(self, (dx, dy): (isize, isize)) -> Self::Output {
        self + (-dx, -dy)
    }
}

impl std::ops::Sub<Direction> for Point {
    type Output = Option<Point>;

    fn sub(self, rhs: Direction) -> Self::Output {
        self + (-rhs)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl<T> From<(T, T)> for Point
where
    T: Into<usize>,
{
    fn from((x, y): (T, T)) -> Self {
        Point::new(x.into(), y.into())
    }
}
