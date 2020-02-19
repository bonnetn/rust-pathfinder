#[cfg(test)]
use grid_pathfinding::find_path;
use ndarray::{Array2, Ix2, ArrayView2};

fn distance(a: &Ix2, b: &Ix2) -> i32 {
    let (ax, ay) = (a[0] as i32, a[1] as i32);
    let (bx, by) = (b[0] as i32, b[1] as i32);
    return (ax - bx).abs() + (ay - by).abs();
}

fn assert_valid_path(path: &[Ix2], start: &Ix2, end: &Ix2, obstacles: ArrayView2<bool>) {
    assert_eq!(path.is_empty(), false);
    assert_eq!(path.first().unwrap(), start);
    assert_eq!(path.last().unwrap(), end);
    for window in path.windows(2) {
        assert_eq!(false, obstacles[window[0]]);
        assert_eq!(false, obstacles[window[1]]);
        assert_eq!(1, distance(&window[0], &window[1]))
    }
}

#[test]
fn happy_path_obstacles() -> Result<(), Box<dyn std::error::Error>> {
    let start = Ix2(1, 0);
    let end = Ix2(1, 4);

    let mut arr = Array2::from_elem((5, 5), false);
    arr[Ix2(0, 2)] = true;
    arr[Ix2(1, 2)] = true;
    arr[Ix2(2, 2)] = true;
    arr[Ix2(3, 2)] = true;

    let got = find_path(arr.view(), &start, &end)?;
    assert_eq!(got.len(), 11);
    assert_valid_path(&got, &start, &end, arr.view());
    Ok(())
}

#[test]
fn happy_path_no_obstacle() -> Result<(), Box<dyn std::error::Error>> {
    let start = Ix2(1, 0);
    let end = Ix2(1, 4);

    let arr = Array2::from_elem((5, 5), false);

    let got = find_path(arr.view(), &start, &end)?;
    let want = vec![Ix2(1, 0), Ix2(1, 1), Ix2(1, 2), Ix2(1, 3), Ix2(1, 4)];
    assert_eq!(got, want);
    assert_valid_path(&got, &start, &end, arr.view());
    Ok(())
}

#[test]
fn no_path() {
    let start = Ix2(1, 0);
    let end = Ix2(1, 4);

    let mut arr = Array2::from_elem((5, 5), false);
    arr[Ix2(0, 2)] = true;
    arr[Ix2(1, 2)] = true;
    arr[Ix2(2, 2)] = true;
    arr[Ix2(3, 2)] = true;
    arr[Ix2(4, 2)] = true;

    let result = find_path(arr.view(), &start, &end);
    assert_eq!(true, result.is_err())
}

#[test]
fn end_is_in_obstacle() {
    let start = Ix2(1, 0);
    let end = Ix2(1, 4);

    let mut arr = Array2::from_elem((5, 5), false);
    arr[end] = true;

    let result = find_path(arr.view(), &start, &end);
    assert_eq!(true, result.is_err())
}

#[test]
fn start_is_in_obstacle() {
    let start = Ix2(1, 0);
    let end = Ix2(1, 4);

    let mut arr = Array2::from_elem((5, 5), false);
    arr[start] = true;

    let result = find_path(arr.view(), &start, &end);
    assert_eq!(true, result.is_err())
}
