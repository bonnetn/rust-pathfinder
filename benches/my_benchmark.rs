use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::{Array2, Ix2};
use rand::Rng;

const WIDTH: usize = 300;
const HEIGHT: usize = 200;

fn make_wall(obstacles: &mut Array2<bool>, x: usize) {
    let (_, height) = obstacles.dim();
    for y in 0..height {
        obstacles[(x, y)] = true;
    }
}

fn make_snail_obstacle_map(start: &Ix2, end: &Ix2) -> Array2<bool> {
    let mut arr = Array2::from_elem((50, 50), false);

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
    let start = Ix2(49, 49);
    let end = Ix2(0, 0);
    let obstacles = make_snail_obstacle_map(&start, &end);

    c.bench_function("find path with snail map (50x50)", |b| {
        b.iter(|| {
            grid_pathfinding::find_path_impl(
                black_box(obstacles.view()),
                black_box(&start),
                black_box(&end),
            )
        })
    });
}

fn find_in_empty_map(c: &mut Criterion) {
    let start = Ix2(WIDTH - 1, HEIGHT - 1);
    let end = Ix2(0, 0);
    let obstacles = Array2::from_elem((WIDTH, HEIGHT), false);

    c.bench_function("find path in empty map", |b| {
        b.iter(|| {
            grid_pathfinding::find_path_impl(
                black_box(obstacles.view()),
                black_box(&start),
                black_box(&end),
            )
        })
    });
}

fn random_map(c: &mut Criterion) {
    let start = Ix2(WIDTH - 1, HEIGHT - 1);
    let end = Ix2(0, 0);

    let obstacles = {
        let mut arr = Array2::from_elem((WIDTH, HEIGHT), false);
        let mut rng = rand::thread_rng();
        for cell in arr.iter_mut() {
            *cell = rng.gen_bool(0.3);
        }
        arr[start] = false;
        arr[end] = false;
        arr
    };

    c.bench_function("find path in random map", |b| {
        b.iter(|| {
            let result = grid_pathfinding::find_path_impl(
                black_box(obstacles.view()),
                black_box(&start),
                black_box(&end),
            );
            assert_eq!(false, result.is_err() );
        })
    });
}

criterion_group!(benches, find_in_empty_map, find_in_snail_map, random_map);
criterion_main!(benches);
