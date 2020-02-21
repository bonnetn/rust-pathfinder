use std::cmp::Ordering::Equal;
use std::collections::BinaryHeap;
use std::f64::INFINITY;

use ndarray::{Array, Array2, ArrayView2, Ix2};

use crate::errors::NoPathFoundError;
use crate::heap::HeapElement;
use crate::line_of_sight::line_of_sight;
use crate::neighbors::get_neighbors;

pub fn find_path<'a>(obstacles: ArrayView2<'a, bool>, start: &'a Ix2, end: &'a Ix2) -> Result<Vec<Ix2>, Box<dyn std::error::Error>> {
    if obstacles[*start] || obstacles[*end] {
        return Err(Box::new(NoPathFoundError()));
    }
    Pathfinder::new(obstacles).find_path(end, start)
}

struct Pathfinder<'a> {
    obstacles: ArrayView2<'a, bool>,
    open_set: BinaryHeap<HeapElement>,
    came_from: Array2<Option<Ix2>>,
    g_score: Array2<f64>,
    f_score: Array2<f64>,
}

fn euclidean_distance(a: &Ix2, b: &Ix2) -> f64 {
    let (ax, ay) = (a[0] as f64, a[1] as f64);
    let (bx, by) = (b[0] as f64, b[1] as f64);
    ((ax - bx) * (ax - bx) + (ay - by) * (ay - by)).sqrt()
}

impl<'a> Pathfinder<'a> {
    fn new(obstacles: ArrayView2<'a, bool>) -> Pathfinder<'a> {
        let open_set = BinaryHeap::with_capacity(1024);
        let came_from = Array::from_elem(obstacles.raw_dim(), None);
        let g_score = Array::from_elem(obstacles.raw_dim(), INFINITY);
        let f_score = Array::from_elem(obstacles.raw_dim(), INFINITY);

        Pathfinder {
            obstacles,
            open_set,
            came_from,
            g_score,
            f_score,
        }
    }


    fn find_path(&mut self, start: &Ix2, end: &Ix2) -> Result<Vec<Ix2>, Box<dyn std::error::Error>> {
        let heuristic = |pos: &Ix2| euclidean_distance(pos, end);
        self.open_set.push(HeapElement {
            position: *start,
            f_score: heuristic(start),
        });
        self.g_score[*start] = 0.;
        self.f_score[*start] = heuristic(start);
        self.came_from[*start] = Some(*start);

        while let Some(HeapElement { position, f_score }) = self.open_set.pop() {
            if f_score > self.f_score[position] { continue; }

            let parent = self.came_from[position].unwrap();
            if !line_of_sight(&position, &parent, &self.obstacles) {
                let (neighbor, g_score) = get_neighbors(&position, self.obstacles.dim())
                    .map(|pos| (pos, self.g_score[pos]))
                    .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap_or(Equal))
                    .unwrap();

                self.g_score[position] = g_score + 1.;
                self.came_from[position] = Some(neighbor);
            }

            self.visit(position, heuristic);

            if position == *end {
                return Ok(self.build_path(start, end));
            }
        }
        Err(Box::new(NoPathFoundError()))
    }

    fn visit<Func: Fn(&Ix2) -> f64>(&mut self, current: Ix2, heuristic: Func) {
        for neighbor in get_neighbors(&current, self.obstacles.dim()) {
            if self.obstacles[neighbor] { continue; }

            let parent = self.came_from[current].unwrap();
            let tentative_g_score = self.g_score[parent] + euclidean_distance(&neighbor, &parent);
            if tentative_g_score < self.g_score[neighbor] {
                self.came_from[neighbor] = Some(parent);
                self.g_score[neighbor] = tentative_g_score;

                let f_score = tentative_g_score + heuristic(&neighbor);
                self.f_score[neighbor] = f_score;
                self.open_set.push(HeapElement { position: neighbor, f_score });
            }
        };
    }

    fn build_path(&self, start: &Ix2, end: &Ix2) -> Vec<Ix2> {
        let mut path = vec![*end];
        let mut pos = *end;
        while pos != *start {
            pos = self.came_from[pos].unwrap();
            path.push(pos);
        }
        path
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path_obstacles() -> Result<(), Box<dyn std::error::Error>> {
        let start = Ix2(0, 5);
        let end = Ix2(5, 5);

        let mut arr = Array2::from_elem((6, 6), false);
        arr[Ix2(2, 1)] = true;
        arr[Ix2(2, 2)] = true;
        arr[Ix2(2, 3)] = true;
        arr[Ix2(2, 4)] = true;
        arr[Ix2(2, 5)] = true;

        arr[Ix2(4, 0)] = true;
        arr[Ix2(4, 1)] = true;
        arr[Ix2(4, 2)] = true;
        arr[Ix2(4, 3)] = true;
        arr[Ix2(4, 4)] = true;

        let got = find_path(arr.view(), &start, &end)?;
        assert_eq!(vec![Ix2(0, 5), Ix2(2, 0), Ix2(3, 1), Ix2(4, 5), Ix2(5, 5)], got);
        Ok(())
    }

    #[test]
    fn happy_path_no_obstacle() -> Result<(), Box<dyn std::error::Error>> {
        let start = Ix2(1, 0);
        let end = Ix2(4, 4);

        let arr = Array2::from_elem((5, 5), false);

        let got = find_path(arr.view(), &start, &end)?;
        let want = vec![Ix2(1, 0), Ix2(4, 4)];
        assert_eq!(got, want);
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
