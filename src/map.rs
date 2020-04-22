use crate::grid::Grid;
use crate::point::Point2D;
use crate::line_of_sight::line_of_sight;

pub trait Bounded {
    fn boundaries(&self) -> (Point2D, Point2D);
}

pub trait Map: Bounded {
    fn obstacle(&self, point: &Point2D) -> bool;
    fn line_of_sight(&self, start: &Point2D, end: &Point2D) -> bool;
}

pub struct GridMap {
    obstacles: Grid<bool>,
}

impl GridMap {
    pub fn new(obstacles: Grid<bool>) -> GridMap {
        GridMap { obstacles }
    }
}

impl Map for GridMap {
    fn obstacle(&self, point: &(isize, isize)) -> bool {
        self.obstacles.get(point)
    }

    fn line_of_sight(&self, start: &(isize, isize), end: &(isize, isize)) -> bool {
        line_of_sight(start, end, &self.obstacles)
    }
}

impl Bounded for GridMap {
    fn boundaries(&self) -> ((isize, isize), (isize, isize)) {
        return self.obstacles.boundaries();
    }
}

