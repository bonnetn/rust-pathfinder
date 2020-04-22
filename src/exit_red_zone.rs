use std::collections::BinaryHeap;

use crate::errors::NoPathFoundError;
use crate::grid::Grid;
use crate::heap::HeapElement;
use crate::neighbors::get_neighbors;
use crate::point::{euclidean_distance, Point2D};
use crate::map::Map;

pub fn exit_red_zone_impl(map: &impl Map, start: &Point2D) -> Result<Point2D, Box<dyn std::error::Error>> {
    let mut already_added_to_queue = Grid::new(map.boundaries(), false);
    if !map.obstacle(start) {
        return Ok(*start);
    }
    let mut q = BinaryHeap::with_capacity(1024);
    q.push(HeapElement { f_score: 0.0, position: *start });

    while let Some(HeapElement { position, f_score: _ }) = q.pop() {
        if !map.obstacle(&position) {
            return Ok(position);
        }

        for n in get_neighbors(position, map.boundaries()) {
            if !already_added_to_queue.get(&n) {
                q.push(HeapElement { f_score: euclidean_distance(&start, &n), position: n });
                already_added_to_queue.set(&n, true);
            }
        }
    }
    Err(Box::new(NoPathFoundError()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::{Bounded, Map};

    struct StubMap {
        exit_cell: Option<Point2D>,
    }

    impl Map for StubMap {
        fn obstacle(&self, point: &Point2D) -> bool {
            self.exit_cell.is_none() || self.exit_cell.unwrap() != *point
        }

        fn line_of_sight(&self, _: &(isize, isize), _: &(isize, isize)) -> bool {
            false
        }
    }

    impl Bounded for StubMap {
        fn boundaries(&self) -> (Point2D, Point2D) {
            ((-20, -20), (20, 20))
        }
    }

    #[test]
    fn no_exit_cell() -> Result<(), Box<dyn std::error::Error>> {
        let result = exit_red_zone_impl(&StubMap {
            exit_cell: None,
        }, &(-10, -10));
        assert_eq!(result.is_err(), true);
        Ok(())
    }

    #[test]
    fn happy_path() -> Result<(), Box<dyn std::error::Error>> {
        let result = exit_red_zone_impl(&StubMap {
            exit_cell: Some((4, 2)),
        }, &(-10, -10));
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), (4, 2));
        Ok(())
    }
}

