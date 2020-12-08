from typing import List
from collections import namedtuple
from functools import reduce

grid: List[List[bool]] = []
for line in open('../inputs/3'):
    row = list(map(lambda c: c == '#', line.strip()))
    grid.append(row)

Slope = namedtuple('Slope', 'right down')

def num_collisions(s: Slope) -> int:
    count = 0
    y = 0
    x = 0
    while y < len(grid):
        count += grid[y][x % len(grid[y])]
        y += s.down
        x += s.right
    return count

s = Slope(3, 1)
print(num_collisions(s))

pairs = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
slopes = map(lambda p: Slope(*p), pairs)
collisions = map(num_collisions, slopes)
product = reduce(int.__mul__, collisions)
print(product)
