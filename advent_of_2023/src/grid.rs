#![allow(dead_code)]

use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    inner: Vec<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GridIndex {
    row: usize,
    column: usize,
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            inner: Default::default(),
        }
    }
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_dimensions(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            inner: Vec::with_capacity(width * height),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn append_row(&mut self) {
        unimplemented!()
    }

    pub fn append_column(&mut self) {
        unimplemented!()
    }

    pub fn append_row_with_data(&mut self, row: &[T]) {
        unimplemented!()
    }

    pub fn append_column_with_data(&mut self, column: &[T]) {
        unimplemented!()
    }

    pub fn insert_row(&mut self, index: usize) {
        unimplemented!()
    }

    pub fn insert_column(&mut self, index: usize) {
        unimplemented!()
    }

    pub fn insert_row_with_data(&mut self, index: usize, row: &[T]) {
        unimplemented!()
    }

    pub fn insert_column_with_data(&mut self, index: usize, column: &[T]) {
        unimplemented!()
    }

    fn row_major_index(&self, index: GridIndex) -> usize {
        index.row + index.column * self.width
    }
}

impl<T, I: Into<GridIndex>> Index<I> for Grid<T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.inner.index(self.row_major_index(index.into()))
    }
}

impl<T, I: Into<GridIndex>> IndexMut<I> for Grid<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.inner.index_mut(self.row_major_index(index.into()))
    }
}

impl From<(usize, usize)> for GridIndex {
    fn from((row, column): (usize, usize)) -> Self {
        Self { row, column }
    }
}
