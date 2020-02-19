
use std::collections::BinaryHeap;
use crate::queue::HeapElement;
use crate::errors::NoPathFoundError;
use crate::neighbors::get_neighbors;
use ndarray::{Array, Array2, Ix2, ArrayView2};
use std::f64::INFINITY;


struct Pathfinder<'a> {
    obstacles: ArrayView2<'a, bool>,
    start: &'a Ix2,
    end: &'a Ix2,

    distance_to_origin: Array2<f64>,
    prev: Array2<Option<Ix2>>,
    heap: BinaryHeap<HeapElement>,
}

impl<'a> Pathfinder<'a> {
    fn new(obstacles: ArrayView2<'a, bool>, start: &'a Ix2, end: &'a Ix2) -> Pathfinder<'a> {
        let distance_to_origin = Array::from_elem(obstacles.raw_dim(), INFINITY);
        let prev = Array::from_elem(obstacles.raw_dim(), None);
        let heap = BinaryHeap::new();

        Pathfinder { distance_to_origin, obstacles, prev, heap, start, end }
    }

    fn find_path(&mut self) -> Result<Vec<Ix2>, Box<dyn std::error::Error>> {
        self.distance_to_origin[*self.end] = 0.;
        self.heap.push(HeapElement { distance: 0., position: *self.end });

        while let Some(HeapElement { position, distance }) = self.heap.pop() {
            self.visit(position, distance);
            if position == *self.start {
                return Ok(self.build_path());
            }
        }
        return Err(Box::new(NoPathFoundError()));
    }

    fn visit(&mut self, position: Ix2, distance: f64) {
        let cost = self.distance_to_origin[position];
        if cost < distance {
            return; // Already a better alternative was calculated.
        }


        for n in get_neighbors(&position, self.distance_to_origin.dim()) {
            if !self.obstacles[n] {
                self.add_to_visit_heap(n, distance + 1., position)
            }
        };
    }

    fn add_to_visit_heap(&mut self, position: Ix2, distance: f64, previous_pos: Ix2) {
        let cost = self.distance_to_origin[position];
        if cost > distance {
            self.distance_to_origin[position] = distance;
            self.prev[position] = Some(previous_pos);
            self.heap.push(HeapElement {
                distance,
                position,
            });
        }
    }

    fn build_path(&self) -> Vec<Ix2> {
        let start = *self.start;
        let mut path = vec![start];
        let mut pos = start;
        while pos != *self.end {
            pos = self.prev[pos].unwrap();
            path.push(pos);
        }
        path
    }
}


pub(crate) fn find_path<'a>(obstacles: ArrayView2<'a, bool>, start: &'a Ix2, end: &'a Ix2) -> Result<Vec<Ix2>, Box<dyn std::error::Error>> {
    if obstacles[*start] || obstacles[*end] {
        return Err(Box::new(NoPathFoundError()));
    }
    Pathfinder::new(obstacles, start, end).find_path()
}


#[cfg(test)]
mod tests {
    use crate::pathfinder::find_path;
    use ndarray::{Array2, Ix2, ArrayView2};

    fn distance(a: &Ix2, b: &Ix2) -> i32 {
        let (ax, ay) = (a[0] as i32, a[1] as i32);
        let (bx, by) = (b[0] as i32, b[1] as i32);
        return (ax - bx).abs() + (ay - by).abs();
    }

    fn assert_valid_path(path: &[Ix2], start: &Ix2, end: &Ix2, obstacles: ArrayView2<bool>) {
        assert_eq!(path.is_empty(), false);
        assert_eq!(path.first().unwrap(), start);
        assert_eq!(path.last().unwrap(), end);
        for window in path.windows(2) {
            assert_eq!(false, obstacles[window[0]]);
            assert_eq!(false, obstacles[window[1]]);
            assert_eq!(1, distance(&window[0], &window[1]))
        }
    }

    #[test]
    fn happy_path_obstacles() -> Result<(), Box<dyn std::error::Error>> {
        let start = Ix2(1, 0);
        let end = Ix2(1, 4);

        let mut arr = Array2::from_elem((5, 5), false);
        arr[Ix2(0, 2)] = true;
        arr[Ix2(1, 2)] = true;
        arr[Ix2(2, 2)] = true;
        arr[Ix2(3, 2)] = true;

        let got = find_path(arr.view(), &start, &end)?;
        assert_eq!(got.len(), 11);
        assert_valid_path(&got, &start, &end, arr.view());
        Ok(())
    }

    #[test]
    fn happy_path_no_obstacle() -> Result<(), Box<dyn std::error::Error>> {
        let start = Ix2(1, 0);
        let end = Ix2(1, 4);

        let arr = Array2::from_elem((5, 5), false);

        let got = find_path(arr.view(), &start, &end)?;
        let want = vec![Ix2(1, 0), Ix2(1, 1), Ix2(1, 2), Ix2(1, 3), Ix2(1, 4)];
        assert_eq!(got, want);
        assert_valid_path(&got, &start, &end, arr.view());
        Ok(())
    }

    #[test]
    fn no_path() {
        let start = Ix2(1, 0);
        let end = Ix2(1, 4);

        let mut arr = Array2::from_elem((5, 5), false);
        arr[Ix2(0, 2)] = true;
        arr[Ix2(1, 2)] = true;
        arr[Ix2(2, 2)] = true;
        arr[Ix2(3, 2)] = true;
        arr[Ix2(4, 2)] = true;

        let result = find_path(arr.view(), &start, &end);
        assert_eq!(true, result.is_err())
    }

    #[test]
    fn end_is_in_obstacle() {
        let start = Ix2(1, 0);
        let end = Ix2(1, 4);

        let mut arr = Array2::from_elem((5, 5), false);
        arr[end] = true;

        let result = find_path(arr.view(), &start, &end);
        assert_eq!(true, result.is_err())
    }

    #[test]
    fn start_is_in_obstacle() {
        let start = Ix2(1, 0);
        let end = Ix2(1, 4);

        let mut arr = Array2::from_elem((5, 5), false);
        arr[start] = true;

        let result = find_path(arr.view(), &start, &end);
        assert_eq!(true, result.is_err())
    }

}


