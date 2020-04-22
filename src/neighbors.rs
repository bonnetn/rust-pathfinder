use crate::point::Point2D;

fn right((pos_x, pos_y): &Point2D, (_, max): &(Point2D, Point2D)) -> Option<Point2D> {
    let (bound, _) = max;
    if *pos_x + 1 >= *bound { return None; }
    Some((pos_x + 1, *pos_y))
}

fn up((pos_x, pos_y): &Point2D, (_, max): &(Point2D, Point2D)) -> Option<Point2D> {
    let (_, bound) = max;
    if *pos_y + 1 >= *bound { return None; }
    Some((*pos_x, pos_y + 1))
}

fn left((pos_x, pos_y): &Point2D, (min, _): &(Point2D, Point2D)) -> Option<Point2D> {
    let (bound, _) = min;
    if *pos_x - 1 < *bound { return None; }
    Some((pos_x - 1, *pos_y))
}

fn down((pos_x, pos_y): &Point2D, (min, _): &(Point2D, Point2D)) -> Option<Point2D> {
    let (_, bound) = min;
    if *pos_y - 1 < *bound { return None; }
    Some((*pos_x, pos_y - 1))
}

fn upright(pos: &Point2D, boundaries: &(Point2D, Point2D)) -> Option<Point2D> {
    right(&up(pos, boundaries)?, boundaries)
}

fn upleft(pos: &Point2D, boundaries: &(Point2D, Point2D)) -> Option<Point2D> {
    left(&up(pos, boundaries)?, boundaries)
}

fn downright(pos: &Point2D, boundaries: &(Point2D, Point2D)) -> Option<Point2D> {
    right(&down(pos, boundaries)?, boundaries)
}

fn downleft(pos: &Point2D, boundaries: &(Point2D, Point2D)) -> Option<Point2D> {
    left(&down(pos, boundaries)?, boundaries)
}

type GetNeighborFunc = fn(&Point2D, &(Point2D, Point2D)) -> Option<Point2D>;

const NEIGHBOR_FUNCS: [GetNeighborFunc; 8] = [
    right, up, left, down,
    upright, upleft, downleft, downright,
];


pub(crate) fn get_neighbors(pos: Point2D, boundaries: (Point2D, Point2D)) -> impl Iterator<Item=Point2D> {
    NEIGHBOR_FUNCS.iter()
        .filter_map(move |func| func(&pos, &boundaries))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn assert_same_elements(got: &HashSet<Point2D>, want: &[Point2D]) {
        assert_eq!(got.len(), want.len());
        for pos in want.iter() {
            assert_eq!(got.contains(&pos), true);
        }
    }

    #[test]
    fn test_neighbors_happy_path() {
        let neighbors: HashSet<Point2D> = get_neighbors((1, 1), ((0, 0), (100, 100))).collect();
        let want = [
            (2, 1),
            (1, 2),
            (0, 1),
            (1, 0),
            (2, 2),
            (0, 2),
            (0, 0),
            (2, 0),
        ];
        assert_same_elements(&neighbors, &want);
    }

    #[test]
    fn test_neighbors_in_top_left_corner() {
        let neighbors: HashSet<Point2D> = get_neighbors((99, 99), ((0, 0), (100, 100))).collect();
        let want = [
            (98, 99),
            (99, 98),
            (98, 98),
        ];
        assert_same_elements(&neighbors, &want);
    }

    #[test]
    fn test_neighbors_in_bottom_right_corner() {
        let neighbors: HashSet<Point2D> = get_neighbors((0, 0), ((0, 0), (100, 100))).collect();
        let want = [
            (1, 1),
            (1, 0),
            (0, 1),
        ];
        assert_same_elements(&neighbors, &want);
    }
}
