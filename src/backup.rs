use ndarray::{Array2, Ix2, Array};

mod queue;
mod errors;
mod pathfinder;
mod neighbors;


extern crate ndarray;

fn make_wall(obstacles: &mut Array2<bool>, x: usize) {
    let (_, height) = obstacles.dim();
    for y in 0..height {
        obstacles[(x,y)] = true;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Ix2(49, 49);
    let end = Ix2(0, 0);

    let obstacles: Array2<bool> = {
        let mut arr = Array2::from_elem((50, 50), false);

        /*
        let mut rng = rand::thread_rng();
        for value in arr.iter_mut() {
            *value = rng.gen_bool(0.1);
        }
        */

        let (width, height) = arr.dim();
        for x in (0..width).step_by(4) {
            make_wall(&mut arr, x);
            arr[(x, 0)] = false;
        }
        for x in (2..width).step_by(4) {
            make_wall(&mut arr, x);
            arr[(x, height-1)] = false;
        }

        arr[start] = false;
        arr[end] = false;
        arr
    };


    let path = pathfinder::find_path(
        obstacles.view(),
        &start,
        &end,
    )?;
    println!("Found path: {:?}", path);

    {
        let path_map = {
            let mut arr = Array::from_elem(obstacles.raw_dim(), false);
            for p in path {
                arr[p] = true;
            }
            arr
        };
        let (width, height) = obstacles.dim();
        for y in (0..height).rev() {
            print!("|");
            for x in 0..width {
                if obstacles[(x, y)] {
                    print!("#")
                } else if path_map[(x, y)] {
                    print!("+")
                } else {
                    print!(" ")
                }
                print!(" ")
            }
            print!("|");
            println!();
        }
    }
    Ok(())
}


