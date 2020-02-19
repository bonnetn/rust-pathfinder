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
    if pos[0] <= 0 { return None; }
    Some(pos - Ix2(1, 0))
}

fn down(pos: Ix2, _: &(usize, usize)) -> Option<Ix2> {
    if pos[1] <= 0 { return None; }
    Some(pos - Ix2(0, 1))
}

const NEIGHBOR_FUNCS: [fn(Ix2, &(usize, usize)) -> Option<Ix2>; 4] = [right, up, left, down];


pub(crate) fn get_neighbors<'a>(pos: &'a Ix2, shape: (usize, usize)) -> impl Iterator<Item=Ix2> + 'a {
    NEIGHBOR_FUNCS.iter()
        .filter_map(move |func| func(*pos, &shape))
}
