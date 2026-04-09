use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{Display, Write},
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::{Direction, Point};

pub struct Grid<T: Index<usize>> {
    width: usize,
    height: usize,
    items: RefCell<T>,
}

impl<T: Index<usize>> Grid<T> {
    pub fn new(width: usize, height: usize, items: T) -> Grid<T> {
        Self {
            width,
            height,
            items: RefCell::new(items),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn len(&self) -> usize {
        self.width * self.height
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn in_bounds_point(&self, point: &Point) -> bool {
        self.in_bounds(point.x, point.y)
    }

    // Used for indexing as well
    pub fn cell_at(&self, x: usize, y: usize) -> Option<GridCell<'_, T>> {
        self.in_bounds(x, y)
            .then_some(GridCell { grid: self, x, y })
    }

    pub fn cell_at_point(&self, point: &Point) -> Option<GridCell<'_, T>> {
        self.cell_at(point.x, point.y)
    }

    pub fn value_at(&self, x: usize, y: usize) -> Option<Ref<'_, T::Output>> {
        self.in_bounds(x, y)
            .then(|| Ref::map(self.items.borrow(), |items| &items[y * self.width + x]))
    }

    pub fn iter(&self) -> GridIter<'_, T> {
        GridIter::new(self)
    }

    pub fn find_by_value<F>(&self, pred: F) -> Option<GridCell<'_, T>>
    where
        F: Fn(T::Output) -> bool,
        T::Output: Copy,
    {
        self.iter().find(|cell| pred(*cell.value()))
    }
}

impl<U: Copy> Grid<Vec<U>> {
    pub fn fill(width: usize, height: usize, val: U) -> Self {
        Grid::new(
            width,
            height,
            std::iter::repeat_n(val, width * height).collect(),
        )
    }
}

impl<U: Default> Grid<Vec<U>> {
    pub fn default(width: usize, height: usize) -> Self {
        Grid::new(
            width,
            height,
            std::iter::repeat_with(Default::default)
                .take(width * height)
                .collect(),
        )
    }
}

impl<T: Clone + Index<usize>> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            items: self.items.clone(),
        }
    }
}

impl<U> FromStr for Grid<Vec<U>>
where
    U: FromStr,
{
    type Err = <U as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map(str::len).unwrap_or(0);
        let height = s.lines().count();

        let items = s
            .lines()
            .flat_map(|line| line.split_inclusive(|_| true))
            .map(|s| s.parse::<U>())
            .collect::<Result<Vec<U>, _>>()?;

        Ok(Grid {
            width,
            height,
            items: RefCell::new(items),
        })
    }
}

impl<'a, T: Index<usize>> IntoIterator for &'a Grid<T> {
    type Item = GridCell<'a, T>;

    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
pub struct GridIter<'a, T>
where
    T: Index<usize>,
{
    grid: &'a Grid<T>,
    i: usize,
}

impl<'a, T: Index<usize>> GridIter<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        GridIter { grid, i: 0 }
    }
}

impl<'a, T: Index<usize>> Iterator for GridIter<'a, T> {
    type Item = GridCell<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.i % self.grid.width;
        let y = self.i / self.grid.width;

        self.i += 1;

        self.grid.cell_at(x, y)
    }
}

impl<T: IndexMut<usize>> Grid<T> {
    pub fn value_at_mut(&self, x: usize, y: usize) -> Option<RefMut<'_, T::Output>> {
        self.in_bounds(x, y).then(|| {
            RefMut::map(self.items.borrow_mut(), |items| {
                &mut items[y * self.width + x]
            })
        })
    }
}

impl<T> Display for Grid<T>
where
    T: Index<usize>,
    T::Output: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y != 0 {
                f.write_char('\n')?;
            }
            for x in 0..self.width {
                write!(f, "{}", self.value_at(x, y).unwrap())?;
            }
        }
        Ok(())
    }
}

pub struct GridCell<'a, T: Index<usize>> {
    grid: &'a Grid<T>,
    pub x: usize,
    pub y: usize,
}

impl<T: Index<usize>> GridCell<'_, T> {
    pub fn grid(&self) -> &Grid<T> {
        self.grid
    }

    pub fn value(&self) -> Ref<'_, T::Output> {
        self.grid.value_at(self.x, self.y).unwrap()
    }

    pub fn point(&self) -> Point {
        Point::new(self.x, self.y)
    }

    pub fn up(&self) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            self.grid.cell_at(self.x, self.y - 1)
        }
    }

    pub fn down(&self) -> Option<Self> {
        self.grid.cell_at(self.x, self.y + 1)
    }

    pub fn left(&self) -> Option<Self> {
        if self.x == 0 {
            None
        } else {
            self.grid.cell_at(self.x - 1, self.y)
        }
    }

    pub fn right(&self) -> Option<Self> {
        self.grid.cell_at(self.x + 1, self.y)
    }

    pub fn go(&self, direction: &Direction) -> Option<Self> {
        match direction {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }

    pub fn step(&self, step: &Step) -> Option<Self> {
        // let mut cell = *self;
        // for direction in step {
        //     cell = cell.go(direction)?;
        // }
        // Some(cell)

        step.iter()
            .try_fold(*self, |cell, direction| cell.go(direction))
    }

    pub fn neighbours(&self) -> Vec<Self> {
        let neighbours = vec![
            self.up().and_then(|cell| cell.left()),
            self.up(),
            self.up().and_then(|cell| cell.right()),
            self.left(),
            self.right(),
            self.down().and_then(|cell| cell.left()),
            self.down(),
            self.down().and_then(|cell| cell.right()),
        ];

        neighbours.into_iter().flatten().collect()
    }
}

impl<T: IndexMut<usize>> GridCell<'_, T> {
    pub fn value_mut(&mut self) -> RefMut<'_, T::Output> {
        self.grid.value_at_mut(self.x, self.y).unwrap()
    }
}

impl<T: Index<usize>> Clone for GridCell<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Index<usize>> Copy for GridCell<'_, T> {}

pub type Step = [Direction];
