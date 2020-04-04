from math import sqrt

import grid_pathfinding
import numpy as np
from bresenham import bresenham


def print_grid(grid, start, end, path):
    hs = '\033[0;37;47m|\033[0;0m'
    vs = '\033[0;37;47m-\033[0;0m'
    cs = '\033[0;37;47m \033[0;0m'
    separator = cs.join([vs * 3] * len(grid))
    separator = f"{cs}{separator}{cs}"
    to_print = ""
    for y in range(len(grid[0]))[::-1]:
        to_print += separator + "\n"
        for x in range(len(grid)):
            char = "   "
            if (x, y) == start:
                char = "\033[0;37;42m S \033[0;0m"
            elif (x, y) == end:
                char = "\033[0;37;42m E \033[0;0m"
            elif (x, y) in path and grid[x, y]:
                char = "%%%"
            elif (x, y) in path:
                char = "\033[0;37;42m   \033[0;0m"
            elif grid[x, y]:
                char = "\033[0;37;41m   \033[0;0m"

            to_print += f"{hs}{char}"
        to_print += f"{hs}\n"
    to_print += separator + "\n"
    print(to_print)


if __name__ == '__main__':
    arr = np.zeros((10, 10), dtype=bool)

    x, y = (4, 4)
    for ix, iy in np.ndindex(arr.shape):
        dist = sqrt((ix - x) ** 2 + (iy - y) ** 2)
        if dist < 2.5:
            arr[ix, iy] = True

    start = (0, 0)
    end = (9, 9)
    print_grid(arr, start, end, [])

    path = grid_pathfinding.find_path(arr, start, end)
    print_grid(arr, start, end, path)

    path2 = []
    for i in range(1, len(path)):
        prev = path[i - 1]
        cur = path[i]
        path2 += bresenham(*cur, *prev)
    print_grid(arr, start, end, path2)

    exit_point = grid_pathfinding.exit_red_zone(arr, (4,4))
    print_grid(arr, (4,4), exit_point, [])
