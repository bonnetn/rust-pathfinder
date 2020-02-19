use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::{Array2,Ix2};
use grid_pathfinding::find_path;

fn make_wall(obstacles: &mut Array2<bool>, x: usize) {
    let (_, height) = obstacles.dim();
    for y in 0..height {
        obstacles[(x, y)] = true;
    }
}

fn make_snail_obstacle_map(start: &Ix2, end: &Ix2) -> Array2<bool> {
    let mut arr = Array2::from_elem((100, 100), false);

    let (width, height) = arr.dim();
    for x in (0..width).step_by(4) {
        make_wall(&mut arr, x);
        arr[(x, 0)] = false;
    }
    for x in (2..width).step_by(4) {
        make_wall(&mut arr, x);
        arr[(x, height - 1)] = false;
    }

    arr[*start] = false;
    arr[*end] = false;
    arr
}

fn find_in_snail_map(c: &mut Criterion) {
    let start = Ix2(99, 99);
    let end = Ix2(0, 0);
    let obstacles = make_snail_obstacle_map(&start, &end);

    c.bench_function("find path", |b| {
        b.iter(|| {
            grid_pathfinding\::find_path(
                black_box(obstacles.view()),
                black_box(&start),
                black_box(&end),
            )
        })
    });
}
fn find_in_empty_map(c: &mut Criterion) {
    let start = Ix2(99, 99);
    let end = Ix2(0, 0);
    let obstacles = Array2::from_elem((100,100), false);

    c.bench_function("find path", |b| {
        b.iter(|| {
            grid_pathfinding\::find_path(
                black_box(obstacles.view()),
                black_box(&start),
                black_box(&end),
            )
        })
    });
}

criterion_group!(benches, find_in_empty_map, find_in_snail_map);
criterion_main!(benches);
