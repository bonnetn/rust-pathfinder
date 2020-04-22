extern crate grid_pathfinding;


use criterion::{black_box, Criterion, criterion_group, criterion_main};
use ndarray::{Array2, ArrayView2};
use orbclient::{Color, EventOption, Renderer, Window};

use grid_pathfinding::{Grid, GridMap};

fn make_wall(obstacles: &mut Array2<bool>, x: usize) {
    let (_, height) = obstacles.dim();
    for y in 0..height {
        obstacles[(x, y)] = true;
    }
}

fn make_snail_obstacle_map(start: &(usize, usize), end: &(usize, usize), (width, height): (usize, usize)) -> Array2<bool> {
    let mut arr = Array2::from_elem((width, height), false);
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
    const WIDTH: usize = 30;
    const HEIGHT: usize = 20;
    let start = ((WIDTH - 1) as isize, (HEIGHT - 1) as isize);
    let end = (0, 0);
    let obstacles = make_snail_obstacle_map(&(WIDTH - 1, HEIGHT - 1), &(0, 0), (WIDTH, HEIGHT));
    let grid_map = GridMap::new(Grid::from(obstacles));

    c.bench_function(format!("find path in snail map {}x{}", WIDTH, HEIGHT).as_str(), |b| {
        b.iter(|| {
            grid_pathfinding::find_path_impl(
                black_box(&grid_map),
                black_box(start),
                black_box(end),
            )
        })
    });
}

fn find_in_empty_map(c: &mut Criterion) {
    const WIDTH: isize = 300;
    const HEIGHT: isize = 200;
    let start = (WIDTH - 1, HEIGHT - 1);
    let end = (0, 0);
    let obstacles = Array2::from_elem((WIDTH as usize, HEIGHT as usize), false);
    let grid_map = GridMap::new(Grid::from(obstacles));

    c.bench_function(format!("find path in empty map {}x{}", WIDTH, HEIGHT).as_str(), |b| {
        b.iter(|| {
            grid_pathfinding::find_path_impl(
                black_box(&grid_map),
                black_box(start),
                black_box(end),
            )
        })
    });
}


fn find_in_map_with_one_big_obstacle(c: &mut Criterion) {
    const WIDTH: isize = 300;
    const HEIGHT: isize = 200;
    let start = (WIDTH - 1, HEIGHT - 1);
    let end = (0, 0);

    let obstacles = {
        let mut arr = Array2::from_elem((WIDTH as usize, HEIGHT as usize), false);
        fn dist((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> f64 {
            let (ax, ay) = (ax as f64, ay as f64);
            let (bx, by) = (bx as f64, by as f64);
            ((ax - bx) * (ax - bx) + (by - ay) * (by - ay)).sqrt()
        }
        for ((x, y), val) in arr.indexed_iter_mut() {
            if dist((x, y), (150, 100)) < 50. {
                *val = true;
            }
            if dist((x, y), (225, 160)) < 25. {
                *val = true;
            }
            if dist((x, y), (225, 100)) < 25. {
                *val = true;
            }
        }
        arr
    };
    let grid_map = GridMap::new(Grid::from(obstacles));

    c.bench_function(format!("find path in map with one big obstacle {}x{}", WIDTH, HEIGHT).as_str(), |b| {
        b.iter(|| {
            let result = grid_pathfinding::find_path_impl(
                black_box(&grid_map),
                black_box(start),
                black_box(end),
            );
            assert_eq!(false, result.is_err());
        })
    });
}

criterion_group!(benches, find_in_empty_map, find_in_snail_map, find_in_map_with_one_big_obstacle);
criterion_main!(benches);

fn _draw_map((width, height): (u32, u32), obstacles: ArrayView2<bool>, path: &[(isize, isize)]) {
    const CELL_SIZE: u32 = 4;
    const CELL_SIZE_I32: i32 = CELL_SIZE as i32;
    let mut window = Window::new(
        0,
        0,
        width * CELL_SIZE,
        height * CELL_SIZE,
        "TITLE",
    ).unwrap();

    for ((x, y), val) in obstacles.indexed_iter() {
        let (x, y) = (x as i32, y as i32);
        if *val {
            window.rect(x * CELL_SIZE_I32, y * CELL_SIZE_I32, CELL_SIZE, CELL_SIZE, Color::rgb(100, 100, 100));
        }
    }

    for w in path.windows(2) {
        let (prev, next) = (w[0], w[1]);
        let (ax, ay) = (prev.0 as i32, prev.1 as i32);
        window.rect(ax * CELL_SIZE_I32, ay * CELL_SIZE_I32, CELL_SIZE, CELL_SIZE, Color::rgb(255, 0, 0));

        let (bx, by) = (next.0 as i32, next.1 as i32);
        window.rect(bx * CELL_SIZE_I32, by * CELL_SIZE_I32, CELL_SIZE, CELL_SIZE, Color::rgb(255, 0, 0));

        window.line(ax * CELL_SIZE_I32, ay * CELL_SIZE_I32, bx * CELL_SIZE_I32, by * CELL_SIZE_I32, Color::rgb(255, 0, 0))
    }


    window.sync();

    'events: loop {
        for event in window.events() {
            if let EventOption::Quit(_quit_event) = event.to_option() {
                break 'events;
            }
        }
    }
}
