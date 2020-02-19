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
    Pathfinder::new(obstacles, start, end).find_path()
}

struct Pathfinder<'a> {
    obstacles: ArrayView2<'a, bool>,
    start: &'a Ix2,
    end: &'a Ix2,

    open_set: BinaryHeap<HeapElement>,
    came_from: Array2<Option<Ix2>>,
    g_score: Array2<f64>,
    f_score: Array2<f64>,
}

fn distance(a: &Ix2, b: &Ix2) -> i32 {
    let (ax, ay) = (a[0] as i32, a[1] as i32);
    let (bx, by) = (b[0] as i32, b[1] as i32);
    return (ax - bx).abs() + (ay - by).abs();
}

impl<'a> Pathfinder<'a> {
    fn new(obstacles: ArrayView2<'a, bool>, start: &'a Ix2, end: &'a Ix2) -> Pathfinder<'a> {
        let open_set = BinaryHeap::new();
        let came_from = Array::from_elem(obstacles.raw_dim(), None);
        let g_score = Array::from_elem(obstacles.raw_dim(), INFINITY);
        let f_score = Array::from_elem(obstacles.raw_dim(), INFINITY);

        Pathfinder {
            obstacles,
            start:end, // Swapping start and end for more efficient path reconstruction.
            end:start,
            open_set,
            came_from,
            g_score,
            f_score,
        }
    }


    fn h(&self, position: &Ix2) -> f64 {
        return distance(self.end, position) as f64;
        //return 0.;
    }

    fn find_path(&mut self) -> Result<Vec<Ix2>, Box<dyn std::error::Error>> {
        self.open_set.push(HeapElement {
            position: *self.start,
            f_score: self.h(self.start),
        });
        self.g_score[*self.start] = 0.;
        self.f_score[*self.start] = self.h(self.start);

        while let Some(HeapElement { position, f_score }) = self.open_set.pop() {
            if f_score != self.f_score[position] { continue; }

            self.visit(position);

            if position == *self.end {
                return Ok(self.build_path());
            }
        }
        return Err(Box::new(NoPathFoundError()));
    }

    fn visit(&mut self, current: Ix2) {
        for neighbor in get_neighbors(&current, self.obstacles.dim()) {
            if self.obstacles[neighbor] { continue; }
            let tentative_g_score = self.g_score[current] + 1.;
            if tentative_g_score < self.g_score[neighbor] {
                self.came_from[neighbor] = Some(current);
                self.g_score[neighbor] = tentative_g_score;

                let f_score = tentative_g_score + self.h(&neighbor);
                self.f_score[neighbor] = f_score;
                self.open_set.push(HeapElement { position: neighbor, f_score });
            }
        };
    }

    fn build_path(&self) -> Vec<Ix2> {
        let end = *self.end;
        let mut path = vec![end];
        let mut pos = end;
        while pos != *self.start {
            pos = self.came_from[pos].unwrap();
            path.push(pos);
        }
        path
    }
}





