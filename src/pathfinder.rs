use std::collections::BinaryHeap;
use crate::heap::HeapElement;
use crate::errors::NoPathFoundError;
use crate::neighbors::get_neighbors;
use ndarray::{Array, Array2, Ix2, ArrayView2};
use std::f64::INFINITY;
use bresenham::Bresenham;
use std::iter::once;
use std::cmp::Ordering::Equal;

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

fn line_of_sight(a: &Ix2, b: &Ix2, obstacles: &ArrayView2<bool>) -> bool {
    let start = (a[0] as isize, a[1] as isize);
    let end = (b[0] as isize, b[1] as isize);
    return !Bresenham::new(start, end)
        .chain(Bresenham::new(end, start))
        .map(|(x, y)| Ix2(x as usize, y as usize))
        .chain(once(*b))
        .map(|pos| obstacles[pos])
        .any(|has_obstacle| has_obstacle);
}

fn euclidean_distance(a: &Ix2, b: &Ix2) -> f64 {
    let (ax, ay) = (a[0] as f64, a[1] as f64);
    let (bx, by) = (b[0] as f64, b[1] as f64);
    return ((ax - bx) * (ax - bx) + (ay - by) * (ay - by)).sqrt();
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
        let heuristic = |pos: &Ix2| euclidean_distance(pos, end) as f64;
        self.open_set.push(HeapElement {
            position: *start,
            f_score: heuristic(start),
        });
        self.g_score[*start] = 0.;
        self.f_score[*start] = heuristic(start);
        self.came_from[*start] = Some(*start);

        while let Some(HeapElement { position, f_score }) = self.open_set.pop() {
            if f_score != self.f_score[position] { continue; }

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
        return Err(Box::new(NoPathFoundError()));
    }

    fn visit<Func: Fn(&Ix2) -> f64>(&mut self, current: Ix2, heuristic: Func) {
        for neighbor in get_neighbors(&current, self.obstacles.dim()) {
            if self.obstacles[neighbor] { continue; }

            let parent = self.came_from[current].unwrap();
            let tentative_g_score = self.g_score[parent] + euclidean_distance(&current, &parent);
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





