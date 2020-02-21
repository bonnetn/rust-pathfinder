use ndarray::Ix2;

fn right(pos: Ix2, shape: &(usize, usize)) -> Option<Ix2> {
    if pos[0] + 1 >= shape.0 { return None; }
    Some(pos + Ix2(1, 0))
}

fn up(pos: Ix2, shape: &(usize, usize)) -> Option<Ix2> {
    if pos[1] + 1 >= shape.1 { return None; }
    Some(pos + Ix2(0, 1))
}

fn left(pos: Ix2, _: &(usize, usize)) -> Option<Ix2> {
    if pos[0] == 0 { return None; }
    Some(pos - Ix2(1, 0))
}

fn down(pos: Ix2, _: &(usize, usize)) -> Option<Ix2> {
    if pos[1] == 0 { return None; }
    Some(pos - Ix2(0, 1))
}

fn upright(pos: Ix2, shape: &(usize, usize)) -> Option<Ix2> {
    right(up(pos, shape)?, shape)
}

fn upleft(pos: Ix2, shape: &(usize, usize)) -> Option<Ix2> {
    left(up(pos, shape)?, shape)
}

fn downright(pos: Ix2, shape: &(usize, usize)) -> Option<Ix2> {
    right(down(pos, shape)?, shape)
}

fn downleft(pos: Ix2, shape: &(usize, usize)) -> Option<Ix2> {
    left(down(pos, shape)?, shape)
}

type GetNeighborFunc = fn(Ix2, &(usize, usize)) -> Option<Ix2>;

const NEIGHBOR_FUNCS: [GetNeighborFunc; 8] = [
    right, up, left, down,
    upright, upleft, downleft, downright,
];


pub(crate) fn get_neighbors<'a>(pos: &'a Ix2, shape: (usize, usize)) -> impl Iterator<Item=Ix2> + 'a {
    NEIGHBOR_FUNCS.iter()
        .filter_map(move |func| func(*pos, &shape))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn assert_same_elements(got: &HashSet<Ix2>, want: &[Ix2]) {
        assert_eq!(got.len(), want.len());
        for pos in want.iter() {
            assert_eq!(got.contains(&pos), true);
        }
    }

    #[test]
    fn test_neighbors_happy_path() {
        let neighbors: HashSet<Ix2> = get_neighbors(&Ix2(1, 1), (100, 100)).collect();
        let want = [
            Ix2(2, 1),
            Ix2(1, 2),
            Ix2(0, 1),
            Ix2(1, 0),
            Ix2(2, 2),
            Ix2(0, 2),
            Ix2(0, 0),
            Ix2(2, 0),
        ];
        assert_same_elements(&neighbors, &want);
    }

    #[test]
    fn test_neighbors_in_top_left_corner() {
        let neighbors: HashSet<Ix2> = get_neighbors(&Ix2(99, 99), (100, 100)).collect();
        let want = [
            Ix2(98, 99),
            Ix2(99, 98),
            Ix2(98, 98),
        ];
        assert_same_elements(&neighbors, &want);
    }

    #[test]
    fn test_neighbors_in_bottom_right_corner() {
        let neighbors: HashSet<Ix2> = get_neighbors(&Ix2(0, 0), (100, 100)).collect();
        let want = [
            Ix2(1, 1),
            Ix2(1, 0),
            Ix2(0, 1),
        ];
        assert_same_elements(&neighbors, &want);
    }
}
