use crate::grid::Grid;
use crate::line_of_sight::line_of_sight;
use crate::point::Point2D;

pub trait Bounded {
    fn boundaries(&self) -> (Point2D, Point2D);
}

pub trait Map: Bounded {
    fn obstacle(&self, point: &Point2D) -> bool;
    fn line_of_sight(&self, start: &Point2D, end: &Point2D) -> bool;
}

pub struct GridMap {
    obstacles: Grid<bool>,
}

impl GridMap {
    pub fn new(obstacles: Grid<bool>) -> GridMap {
        GridMap { obstacles }
    }
}

impl Map for GridMap {
    fn obstacle(&self, point: &(isize, isize)) -> bool {
        self.obstacles.get(point)
    }

    fn line_of_sight(&self, start: &(isize, isize), end: &(isize, isize)) -> bool {
        line_of_sight(start, end, &self.obstacles)
    }
}

impl Bounded for GridMap {
    fn boundaries(&self) -> ((isize, isize), (isize, isize)) {
        return self.obstacles.boundaries();
    }
}

pub struct GridMap2 {
    obstacles: Grid<bool>,
    circles: Vec<(Point2D, isize)>,
}

impl GridMap2 {
    pub fn new(obstacles: Grid<bool>, circles: Vec<(Point2D, isize)>) -> GridMap2 {
        GridMap2 { obstacles, circles }
    }
}

fn norm2(x: isize, y: isize) -> isize {
    return x * x + y * y;
}

fn sub((ax, ay): (isize, isize), (bx, by): (isize, isize)) -> (isize, isize) {
    return (ax - bx, ay - by);
}

impl Map for GridMap2 {
    fn obstacle(&self, point: &(isize, isize)) -> bool {
        self.obstacles.get(point)
    }

    fn line_of_sight(&self, start: &(isize, isize), end: &(isize, isize)) -> bool {
        // intersection = DirRay * t + StarRay
        // || intersection - circlePos || = radius

        // intersection - circlePos = DirRay * t + StarRay - circlePos

        // || intersection - circlePos ||^2 = radius^2 = (intersection - circlePos) . (intersection - circlePos)

        // radius^2 = (DirRay * t + StartRay - CirclePos)(DirRay * t + StarRay - CirclePos)
        // Alpha = StartRay - CirclePos

        // radius^2 = (DirRay * t + Alpha)(DirRay * t + Alpha)

        // radius^2 = t^2 * || DirRay || ^ 2 + 2 * t * (DirRay.Alpha) + || Alpha || ^ 2
        // 0 = t^2 * || DirRay || ^ 2 + 2 * t * (DirRay.Alpha) + || Alpha || ^ 2 - radius ^ 2

        // A = || DirRay || ^ 2
        // B = 2 * (DirRay.Alpha)
        // C = || Alpha || ^ 2 - radius ^ 2

        // Delta = B^2 - 4 * A * C
        // X1 = (-B-sqrt(Delta))/2/A
        // X2 = (-B+sqrt(Delta))/2/A

        let (dir_ray_x, dir_ray_y) = sub(*end, *start);
        let norm2_dir_ray = norm2(dir_ray_x, dir_ray_y);

        for (point, radius) in &self.circles {
            let (alpha_x, alpha_y) = sub(*start, *point);
            let norm2_alpha = norm2(alpha_x, alpha_y);
            let dir_ray_dot_start = dir_ray_x * alpha_x + dir_ray_y * alpha_y;

            let a = norm2_dir_ray;
            let b = 2 * dir_ray_dot_start;
            let c = norm2_alpha - radius * radius;

            let a = a as i64;
            let b = b as i64;
            let c = c as i64;

            let delta = b * b - 4 * a * c;
            if delta < 0 {
                continue; // Ray does not hit this obstacle.
            }

            let delta_f64 = delta as f64;
            let delta_f64_sqrt = delta_f64.sqrt();
            let x1 = (-(b as f64) - delta_f64_sqrt) / 2. / (a as f64);
            if x1 > 0. && x1 < 1. {
                return false; // Ray hit the obstacle, no LOS;
            }

            let x2 = (-(b as f64) + delta_f64_sqrt) / 2. / (a as f64);
            if x2 > 0. && x2 < 1. {
                return false; // Ray hit the obstacle, no LOS;
            }
        }
        return true;
    }
}

impl Bounded for GridMap2 {
    fn boundaries(&self) -> ((isize, isize), (isize, isize)) {
        return self.obstacles.boundaries();
    }
}

