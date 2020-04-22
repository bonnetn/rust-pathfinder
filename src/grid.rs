use std::convert::TryFrom;

use ndarray::{Array, Array2};
use ndarray::Ix2;

use crate::map::Bounded;
use crate::point::Point2D;

pub struct Grid<T> {
    arr: Array2<T>,
    min: Point2D,
    max: Point2D,
}

impl<T> Grid<T> where T: Copy {
    pub fn new((min, max): (Point2D, Point2D), elem: T) -> Grid<T> {
        let (min_x, min_y) = min;
        let (max_x, max_y) = max;
        let (shape_x, shape_y) = (max_x - min_x, max_y - min_y);
        let shape = (
            usize::try_from(shape_x).unwrap(),
            usize::try_from(shape_y).unwrap(),
        );
        let arr = Array::from_elem(shape, elem);
        Grid {
            arr,
            min,
            max,
        }
    }

    pub fn from(arr: Array2<T>) -> Grid<T> {
        let shape = arr.shape();
        let max = (shape[0] as isize, shape[1] as isize);
        Grid {
            arr,
            min: (0, 0),
            max,
        }
    }

    pub fn get(&self, point: &Point2D) -> T {
        self.arr[self.to_ix2(point)]
    }

    pub fn set(&mut self, point: &Point2D, elem: T) {
        let i = self.to_ix2(point);
        self.arr[i] = elem;
    }

    fn to_ix2(&self, (x, y): &Point2D) -> Ix2 {
        let (min_x, min_y) = self.min;

        let x = x - min_x;
        let x = usize::try_from(x).unwrap();

        let y = y - min_y;
        let y = usize::try_from(y).unwrap();

        Ix2(x, y)
    }
}

impl<T> Bounded for Grid<T> {
    fn boundaries(&self) -> (Point2D, Point2D) {
        (self.min, self.max)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() -> Result<(), Box<dyn std::error::Error>> {
        let mut grid = Grid::new(((-100, 100), (100, 100)), 0);

        grid.set(&(-100, -100), 1);
        grid.set(&(0, 0), 2);
        grid.set(&(99, 99), 3);

        assert_eq!(grid.get(&(-100, -100)), 1);
        assert_eq!(grid.get(&(0, 0)), 2);
        assert_eq!(grid.get(&(99, 99)), 3);
        assert_eq!(grid.get(&(42, -42)), 0);

        Ok(())
    }

    #[test]
    fn test_bounded() -> Result<(), Box<dyn std::error::Error>> {
        let bounds = ((-1, 2), (3, 4));
        let grid = Grid::new(bounds, 0);

        assert_eq!(grid.boundaries(), bounds);

        Ok(())
    }
}

