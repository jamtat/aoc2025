use std::{
    cell::{Ref, RefCell, RefMut},
    ptr,
};

pub struct DoubleBuffer<T> {
    left: RefCell<T>,
    right: RefCell<T>,
}

impl<T> Clone for DoubleBuffer<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

impl<T> Default for DoubleBuffer<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            left: Default::default(),
            right: Default::default(),
        }
    }
}

impl<T> From<T> for DoubleBuffer<T>
where
    T: Clone,
{
    fn from(value: T) -> Self {
        Self {
            left: RefCell::new(value.clone()),
            right: RefCell::new(value),
        }
    }
}

impl<T> From<&T> for DoubleBuffer<T>
where
    T: Clone,
{
    fn from(value: &T) -> Self {
        Self {
            left: RefCell::new(value.clone()),
            right: RefCell::new(value.clone()),
        }
    }
}

impl<T> DoubleBuffer<T> {
    pub fn new(left: T, right: T) -> Self {
        Self {
            left: RefCell::new(left),
            right: RefCell::new(right),
        }
    }

    pub fn left<'a>(&'a self) -> DoubleBufferRef<'a, T> {
        DoubleBufferRef {
            double_buffer: self,
            side: BufferSide::Left,
        }
    }

    pub fn right<'a>(&'a self) -> DoubleBufferRef<'a, T> {
        DoubleBufferRef {
            double_buffer: self,
            side: BufferSide::Right,
        }
    }

    fn ptrs(&self) -> (*mut T, *mut T) {
        (self.left.as_ptr(), self.right.as_ptr())
    }

    pub fn swap(&self) {
        let (l, r) = self.ptrs();
        unsafe {
            ptr::swap(l, r);
        }
    }

    pub fn clone_left(&self)
    where
        T: Clone,
    {
        self.right.replace(self.left.borrow().clone());
    }

    pub fn clone_right(&self)
    where
        T: Clone,
    {
        self.left.replace(self.right.borrow().clone());
    }

    pub fn copy_left(&self)
    where
        T: Copy,
    {
        self.right.replace(*self.left().borrow());
    }

    pub fn copy_right(&self)
    where
        T: Copy,
    {
        self.left.replace(*self.right().borrow());
    }
}

impl<'a, T> std::fmt::Debug for DoubleBufferRef<'a, T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.borrow().fmt(f)
    }
}

impl<'a, T> std::fmt::Display for DoubleBufferRef<'a, T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.borrow().fmt(f)
    }
}

enum BufferSide {
    Left,
    Right,
}

impl BufferSide {
    fn get_cell<'a, T>(&'_ self, double_buffer: &'a DoubleBuffer<T>) -> &'a RefCell<T> {
        match self {
            BufferSide::Left => &double_buffer.left,
            BufferSide::Right => &double_buffer.right,
        }
    }
}

pub struct DoubleBufferRef<'a, T> {
    double_buffer: &'a DoubleBuffer<T>,
    side: BufferSide,
}

impl<'a, T> DoubleBufferRef<'a, T> {
    fn cell(&self) -> &RefCell<T> {
        self.side.get_cell(self.double_buffer)
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.cell().borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.cell().borrow_mut()
    }
}

#[cfg(test)]
mod test {
    use crate::DoubleBuffer;

    #[test]
    fn test_swap() {
        let buffer: DoubleBuffer<usize> = Default::default();
        let left = buffer.left();
        let right = buffer.right();

        *left.borrow_mut() = 1;

        assert_eq!(*left.borrow(), 1);
        assert_eq!(*right.borrow(), 0);

        buffer.swap();

        assert_eq!(*left.borrow(), 0);
        assert_eq!(*right.borrow(), 1);
    }

    #[test]
    fn test_clone_left() {
        let buffer = DoubleBuffer::new(vec![0, 1, 2], vec![100; 100]);
        let left = buffer.left();
        let right = buffer.right();
        buffer.clone_left();
        assert_eq!(*left.borrow(), vec![0, 1, 2]);
        assert_eq!(*right.borrow(), vec![0, 1, 2]);
    }

    #[test]
    fn test_clone_right() {
        let buffer = DoubleBuffer::new(vec![100; 100], vec![0, 1, 2]);
        let left = buffer.left();
        let right = buffer.right();
        buffer.clone_right();
        assert_eq!(*left.borrow(), vec![0, 1, 2]);
        assert_eq!(*right.borrow(), vec![0, 1, 2]);
    }
}
