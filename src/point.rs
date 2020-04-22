pub type Point2D = (isize, isize);

pub fn euclidean_distance((ax, ay): &Point2D, (bx, by): &Point2D) -> f64 {
    let (ax, ay) = (*ax as f64, *ay as f64);
    let (bx, by) = (*bx as f64, *by as f64);
    ((ax - bx) * (ax - bx) + (ay - by) * (ay - by)).sqrt()
}

pub(crate) fn is_in_bounds((x, y): Point2D, ((min_x, min_y), (max_x, max_y)): (Point2D, Point2D)) -> bool {
    x < max_x && y < max_y && x >= min_x && y >= min_y
}
