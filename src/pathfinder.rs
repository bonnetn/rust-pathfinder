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



