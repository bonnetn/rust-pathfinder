import grid_pathfinding
import numpy as np

if __name__ == '__main__':
    p = 1-0.3
    arr = np.random.choice(a=[False, True], size=(10, 10), p=[p, 1 - p])
    print(grid_pathfinding.find_path(arr, (0,0), (9,9)))
