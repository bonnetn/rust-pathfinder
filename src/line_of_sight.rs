use std::iter::once;

use bresenham::Bresenham;
use ndarray::{ArrayView2, Ix2};

pub(crate) fn line_of_sight(a: &Ix2, b: &Ix2, obstacles: &ArrayView2<bool>) -> bool {
    let start = (a[0] as isize, a[1] as isize);
    let end = (b[0] as isize, b[1] as isize);
    !Bresenham::new(start, end)
        .map(|(x, y)| Ix2(x as usize, y as usize))
        .chain(once(*b))
        .map(|pos| obstacles[pos])
        .any(|has_obstacle| has_obstacle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;

    #[test]
    fn test_free_line_of_sight() {
        let obstacles = Array2::from_elem((10, 10), false);

        assert_eq!(line_of_sight(
            &Ix2(0,0),
            &Ix2(9,9),
            &obstacles.view(),
        ), true);
    }
    #[test]
    fn test_blocked_line_of_sight() {
        let obstacles = {
            let mut arr = Array2::from_elem((10, 10), true);
            arr[(0,0)] = false;
            arr[(9,9)] = false;
            arr
        };

        assert_eq!(line_of_sight(
            &Ix2(0,0),
            &Ix2(9,9),
            &obstacles.view(),
        ), false);
    }
}
