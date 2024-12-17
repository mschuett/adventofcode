import textwrap
import collections
from typing import *
from enum import Enum,auto

from .helper import get_input_data

class Direction(Enum):
    N = auto()
    E = auto()
    S = auto()
    W = auto()
    def __str__(self):
        return self.name
    def __repr__(self):
        return self.name

# try if a NamedTuple is faster than a real class
class Coordinate(NamedTuple):
    x: int
    y: int
    def step(self, d: Direction) -> 'Coordinate':
        if d == Direction.N:
            return Coordinate(self.x, self.y - 1)
        elif d == Direction.E:
            return Coordinate(self.x + 1, self.y)
        elif d == Direction.S:
            return Coordinate(self.x, self.y + 1)
        elif d == Direction.W:
            return Coordinate(self.x - 1, self.y)
    def neighbours(self) -> dict['Coordinate', Direction]:
        return {
            Coordinate(self.x, self.y - 1): Direction.N,
            Coordinate(self.x + 1, self.y): Direction.E,
            Coordinate(self.x, self.y + 1): Direction.S,
            Coordinate(self.x - 1, self.y): Direction.W,
        }

class MazePos(NamedTuple):
    pos: Coordinate
    dir: Direction
    # visited: set[Coordinate]
    cost: int

class ReindeerMaze(object):
    width: int
    height: int
    walls: set[Coordinate]
    start: Coordinate
    end: Coordinate
    best_walk: MazePos | None

    def __init__(self, input_text: str) -> None:
        lines = [l.strip() for l in input_text.strip().split('\n')]
        self.width = len(lines[0])
        self.height = len(lines)
        assert self.width == self.height
        assert all([len(l) == self.width for l in lines])
        self.walls = set()
        self.best_walk = None

        for y in range(self.height):
            for x in range(self.width):
                pos = Coordinate(x,y)
                if lines[y][x] == '#':
                    self.walls.add(pos)
                elif lines[y][x] == 'S':
                    self.start = pos
                elif lines[y][x] == 'E':
                    self.end = pos
                else:
                    assert lines[y][x] == '.'

    def __repr__(self) -> str:
        return f'{self.__class__.__name__}(width={self.width}, height={self.height})'

    def __str__(self) -> str:
        return self.prettyprint()

    def prettyprint(self, visited: set[Coordinate] = frozenset()) -> str:
        outstr = self.__repr__() + '\n'
        for y in range(self.height):
            for x in range(self.width):
                pos = Coordinate(x,y)
                if pos in self.walls:
                    outstr += '#'
                elif pos == self.start:
                    outstr += 'S'
                elif pos == self.end:
                    outstr += 'E'
                elif pos in visited:
                    outstr += 'o'
                else:
                    outstr += '.'
            outstr += "\n"
        return outstr

    def is_free(self, pos: Coordinate) -> bool:
        return pos not in self.walls

    def walk(self) -> int:
        working_set = collections.deque([MazePos(self.start, Direction.E, 0)])
        best_walk: MazePos | None = None
        least_cost_to_pos: dict[Coordinate, int] = {}
        all_cell_count = self.width * self.height - len(self.walls)
        iteration = 0

        while working_set:
            cur_pos, cur_dir, cur_cost = working_set.pop()

            if iteration % 1000 == 0:
                print(f'iteration {iteration}, deque size {len(working_set)}, visited cells {len(least_cost_to_pos)}/{all_cell_count}')
            iteration += 1

            if cur_pos not in least_cost_to_pos:
                least_cost_to_pos[cur_pos] = cur_cost
            elif cur_pos in least_cost_to_pos and least_cost_to_pos[cur_pos] + 1001 < cur_cost:
                continue  # current path+turning is longer than alternative, end of path

            if best_walk and best_walk.cost < cur_cost:
                continue  # current path is longer than alternative, end of path
            if cur_pos == self.end:
                if not (best_walk and best_walk.cost < cur_cost):
                    best_walk = MazePos(cur_pos, cur_dir, cur_cost)
                    print(f"found a walk with cost {cur_cost}, remaining deque length is {len(working_set)}")
                continue  # else nothing, end of path

            neighbors = cur_pos.neighbours()
            options = [pos for pos in neighbors.keys()
                       if pos not in self.walls]

            if len(options) == 0:
                continue  # dead end, end of path

            # assert len(options) == 1 or len(options) == 2
            pos_ahead = cur_pos.step(cur_dir)
            for opt in options:
                if opt == pos_ahead:
                    step_cost = 1
                    working_set.append(MazePos(opt, neighbors[opt], cur_cost+step_cost))
                else:
                    step_cost = 1001
                    working_set.appendleft(MazePos(opt, neighbors[opt], cur_cost+step_cost))
        print(f"found path with cost {best_walk.cost}")
        self.best_walk = best_walk
        return best_walk.cost


def solve():
    example = """\
    ###############
    #.......#....E#
    #.#.###.#.###.#
    #.....#.#...#.#
    #.###.#####.#.#
    #.#.#.......#.#
    #.#.#####.###.#
    #...........#.#
    ###.#.#####.#.#
    #...#.....#.#.#
    #.#.#.###.#.#.#
    #.....#...#.#.#
    #.###.#.#.#.#.#
    #S..#.....#...#
    ###############
    """

    # example = """\
    # #################
    # #...#...#...#..E#
    # #.#.#.#.#.#.#.#.#
    # #.#.#.#...#...#.#
    # #.#.#.#.###.#.#.#
    # #...#.#.#.....#.#
    # #.#.#.#.#.#####.#
    # #.#...#.#.#.....#
    # #.#.#####.#.###.#
    # #.#.#.......#...#
    # #.#.###.#####.###
    # #.#.#...#.....#.#
    # #.#.#.#####.###.#
    # #.#.#.........#.#
    # #.#.#.#########.#
    # #S#.............#
    # #################
    # """

    input_text = get_input_data(2024, 16, example)

    # part 1
    m = ReindeerMaze(textwrap.dedent(input_text))
    print(m)
    m.walk()

    # part 2
