#[cfg(test)]
use grid_pathfinding::find_path_impl;
use ndarray::{Array2, Ix2};

#[test]
fn happy_path_obstacles() -> Result<(), Box<dyn std::error::Error>> {
    let start = Ix2(0, 5);
    let end = Ix2(5, 5);

    let mut arr = Array2::from_elem((6, 6), false);
    arr[Ix2(2, 1)] = true;
    arr[Ix2(2, 2)] = true;
    arr[Ix2(2, 3)] = true;
    arr[Ix2(2, 4)] = true;
    arr[Ix2(2, 5)] = true;

    arr[Ix2(4, 0)] = true;
    arr[Ix2(4, 1)] = true;
    arr[Ix2(4, 2)] = true;
    arr[Ix2(4, 3)] = true;
    arr[Ix2(4, 4)] = true;

    let got = find_path_impl(arr.view(), &start, &end)?;
    assert_eq!(vec![Ix2(0, 5), Ix2(2, 0), Ix2(3, 1), Ix2(4, 5), Ix2(5, 5)], got);
    Ok(())
}

#[test]
fn happy_path_no_obstacle() -> Result<(), Box<dyn std::error::Error>> {
    let start = Ix2(1, 0);
    let end = Ix2(4, 4);

    let arr = Array2::from_elem((5, 5), false);

    let got = find_path_impl(arr.view(), &start, &end)?;
    let want = vec![Ix2(1, 0), Ix2(4, 4)];
    assert_eq!(got, want);
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

    let result = find_path_impl(arr.view(), &start, &end);
    assert_eq!(true, result.is_err())
}

#[test]
fn end_is_in_obstacle() {
    let start = Ix2(1, 0);
    let end = Ix2(1, 4);

    let mut arr = Array2::from_elem((5, 5), false);
    arr[end] = true;

    let result = find_path_impl(arr.view(), &start, &end);
    assert_eq!(true, result.is_err())
}

#[test]
fn start_is_in_obstacle() {
    let start = Ix2(1, 0);
    let end = Ix2(1, 4);

    let mut arr = Array2::from_elem((5, 5), false);
    arr[start] = true;

    let result = find_path_impl(arr.view(), &start, &end);
    assert_eq!(true, result.is_err())
}

