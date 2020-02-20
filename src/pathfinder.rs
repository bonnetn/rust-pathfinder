use std::collections::BinaryHeap;
use crate::heap::HeapElement;
use crate::errors::NoPathFoundError;
use crate::neighbors::get_neighbors;
use ndarray::{Array, Array2, Ix2, ArrayView2};
use std::f64::INFINITY;

pub(crate) fn find_path<'a>(obstacles: ArrayView2<'a, bool>, start: &'a Ix2, end: &'a Ix2) -> Result<Vec<Ix2>, Box<dyn std::error::Error>> {
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

fn taxicab_distance(a: &Ix2, b: &Ix2) -> i32 {
    let (ax, ay) = (a[0] as i32, a[1] as i32);
    let (bx, by) = (b[0] as i32, b[1] as i32);
    return (ax - bx).abs() + (ay - by).abs();
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
        let heuristic = |pos: &Ix2| taxicab_distance(pos, end) as f64;

        self.open_set.push(HeapElement {
            position: *start,
            f_score: heuristic(start),
        });
        self.g_score[*start] = 0.;
        self.f_score[*start] = heuristic(start);

        while let Some(HeapElement { position, f_score }) = self.open_set.pop() {
            if f_score != self.f_score[position] { continue; }

            self.visit(position, heuristic);

            if position == *end {
                return Ok(self.build_path(start, end));
            }
        }
        return Err(Box::new(NoPathFoundError()));
    }

    fn visit<Func: Fn(&Ix2) -> f64>(&mut self, current: Ix2, heuristic: Func) {
        for neighbor in get_neighbors(&current, self.obstacles.dim()) {
            if self.obstacles[neighbor] { continue; }
            let tentative_g_score = self.g_score[current] + 1.;
            if tentative_g_score < self.g_score[neighbor] {
                self.came_from[neighbor] = Some(current);
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





