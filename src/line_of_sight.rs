use std::iter::once;

use bresenham::Bresenham;

use crate::map::Map;
use crate::point::Point2D;

pub fn line_of_sight(start: &Point2D, end: &Point2D, map: &impl Map) -> bool {
    !Bresenham::new(*start, *end)
        .chain(once(*end))
        .map(|pos| map.obstacle(&pos))
        .any(|has_obstacle| has_obstacle)
}

#[cfg(test)]
mod tests {
    use ndarray::Array2;

    use crate::grid::Grid;
    use crate::map::GridMap;

    use super::*;

    #[test]
    fn test_free_line_of_sight() {
        let obstacles = Array2::from_elem((10, 10), false);

        assert_eq!(line_of_sight(
            &(0, 0),
            &(9, 9),
            &GridMap::new(Grid::from(obstacles)),
        ), true);
    }

    #[test]
    fn test_blocked_line_of_sight() {
        let obstacles = {
            let mut arr = Array2::from_elem((10, 10), true);
            arr[(0, 0)] = false;
            arr[(9, 9)] = false;
            arr
        };

        assert_eq!(line_of_sight(
            &(0, 0),
            &(9, 9),
            &GridMap::new(Grid::from(obstacles)),
        ), false);
    }
}
