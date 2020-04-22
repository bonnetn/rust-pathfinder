use std::cmp::Ordering::Equal;
use std::collections::BinaryHeap;
use std::f64::INFINITY;

use crate::errors::NoPathFoundError;
use crate::grid::Grid;
use crate::heap::HeapElement;
use crate::map::Map;
use crate::neighbors::get_neighbors;
use crate::point::{euclidean_distance, Point2D};

pub fn find_path_impl(map: &impl Map, start: Point2D, end: Point2D) -> Result<Vec<Point2D>, Box<dyn std::error::Error>> {
    if map.obstacle(&start) || map.obstacle(&end) {
        return Err(Box::new(NoPathFoundError()));
    }
    if map.line_of_sight(&start, &end) {
        return Ok(vec![start, end]);
    }
    do_find_path(map, end, start)
}


fn do_find_path(map: &impl Map, start: Point2D, end: Point2D) -> Result<Vec<Point2D>, Box<dyn std::error::Error>> {
    let mut open_set: BinaryHeap<HeapElement<Point2D>> = BinaryHeap::with_capacity(1024);
    let mut came_from: Grid<Option<Point2D>> = Grid::new(map.boundaries(), None);
    let mut g_score: Grid<f64> = Grid::new(map.boundaries(), INFINITY);
    let mut f_score: Grid<f64> = Grid::new(map.boundaries(), INFINITY);

    let heuristic = |pos: &Point2D| euclidean_distance(pos, &end);

    open_set.push(HeapElement {
        position: start,
        f_score: heuristic(&start),
    });

    g_score.set(&start, 0.);
    f_score.set(&start, heuristic(&start));
    came_from.set(&start, Some(start));

    while let Some(HeapElement { position, f_score: elem_f_score }) = open_set.pop() {
        if elem_f_score > f_score.get(&position) { continue; }

        let parent = came_from.get(&position).unwrap();

        if !map.line_of_sight(&position, &parent) {
            let (neighbor, neighbor_g_score) = get_neighbors(position, map.boundaries())
                .map(|pos| (pos, g_score.get(&pos)))
                .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap_or(Equal))
                .unwrap();

            g_score.set(&position, neighbor_g_score + euclidean_distance(&position, &neighbor) + heuristic(&position));
            came_from.set(&position, Some(neighbor));
        }

        // VISIT:
        for neighbor in get_neighbors(position, map.boundaries()) {
            if map.obstacle(&neighbor) { continue; }

            let parent = came_from.get(&position).unwrap();
            let tentative_g_score = g_score.get(&parent) + euclidean_distance(&neighbor, &parent) + heuristic(&neighbor);
            if tentative_g_score < g_score.get(&neighbor) {
                came_from.set(&neighbor, Some(parent));
                g_score.set(&neighbor, tentative_g_score);

                let new_f_score = tentative_g_score + heuristic(&neighbor) * 2.;
                f_score.set(&neighbor, new_f_score);
                open_set.push(HeapElement { position: neighbor, f_score: new_f_score });
            }
        };

        if position == end {
            return Ok(build_path(&came_from, &start, &end));
        }
    }
    Err(Box::new(NoPathFoundError()))
}


fn build_path(came_from: &Grid<Option<Point2D>>, start: &Point2D, end: &Point2D) -> Vec<Point2D> {
    let mut path = vec![*end];
    let mut pos = *end;
    while pos != *start {
        pos = came_from.get(&pos).unwrap();
        path.push(pos);
    }
    path
}


#[cfg(test)]
mod tests {
    use ndarray::Array2;

    use crate::map::GridMap;

    use super::*;

    #[test]
    fn happy_path_obstacles() -> Result<(), Box<dyn std::error::Error>> {
        let start = (0, 5);
        let end = (5, 5);

        let mut arr = Array2::from_elem((6, 6), false);
        arr[(2, 1)] = true;
        arr[(2, 2)] = true;
        arr[(2, 3)] = true;
        arr[(2, 4)] = true;
        arr[(2, 5)] = true;

        arr[(4, 0)] = true;
        arr[(4, 1)] = true;
        arr[(4, 2)] = true;
        arr[(4, 3)] = true;
        arr[(4, 4)] = true;

        let got = find_path_impl(&GridMap::new(Grid::from(arr)), start, end)?;
        assert_eq!(vec![(0, 5), (2, 0), (3, 1), (3, 5), (5, 5)], got);
        Ok(())
    }

    #[test]
    fn happy_path_no_obstacle() -> Result<(), Box<dyn std::error::Error>> {
        let start = (1, 0);
        let end = (4, 4);

        let arr = Array2::from_elem((5, 5), false);

        let got = find_path_impl(&GridMap::new(Grid::from(arr)), start, end)?;
        let want = vec![(1, 0), (4, 4)];
        assert_eq!(got, want);
        Ok(())
    }

    #[test]
    fn no_path() {
        let start = (1, 0);
        let end = (1, 4);

        let mut arr = Array2::from_elem((5, 5), false);
        arr[(0, 2)] = true;
        arr[(1, 2)] = true;
        arr[(2, 2)] = true;
        arr[(3, 2)] = true;
        arr[(4, 2)] = true;

        let result = find_path_impl(&GridMap::new(Grid::from(arr)), start, end);
        assert_eq!(true, result.is_err())
    }

    #[test]
    fn end_is_in_obstacle() {
        let start = (1, 0);
        let end = (1, 4);

        let mut arr = Array2::from_elem((5, 5), false);
        arr[(1, 4)] = true;

        let result = find_path_impl(&GridMap::new(Grid::from(arr)), start, end);
        assert_eq!(true, result.is_err())
    }

    #[test]
    fn start_is_in_obstacle() {
        let start = (1, 0);
        let end = (1, 4);

        let mut arr = Array2::from_elem((5, 5), false);
        arr[(1, 0)] = true;

        let result = find_path_impl(&GridMap::new(Grid::from(arr)), start, end);
        assert_eq!(true, result.is_err())
    }
}
