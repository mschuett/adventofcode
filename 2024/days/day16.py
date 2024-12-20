import textwrap
from typing import *
import queue
from enum import IntEnum

from .helper import get_input_data

class Direction(IntEnum):
    N = 0
    E = 1
    S = 2
    W = 3
    def __str__(self):
        return self.name
    def __repr__(self):
        return self.name
    def turn_right(self) -> "Direction":
        return Direction((self.value + 1) % 4)
    def turn_left(self) -> "Direction":
        return Direction((self.value + 3) % 4)
    def turn_back(self) -> "Direction":
        return Direction((self.value + 2) % 4)

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
        else:
            raise ValueError()
    def neighbours(self) -> dict['Coordinate', Direction]:
        return {
            Coordinate(self.x, self.y - 1): Direction.N,
            Coordinate(self.x + 1, self.y): Direction.E,
            Coordinate(self.x, self.y + 1): Direction.S,
            Coordinate(self.x - 1, self.y): Direction.W,
        }
    def distance(self, other: 'Coordinate') -> int:
        return abs(self.x - other.x) + abs(self.y - other.y)

class MazePos(NamedTuple):
    dist: int
    cost: int
    pos: Coordinate
    dir: Direction

class ReindeerMaze(object):
    width: int
    height: int
    walls: set[Coordinate]
    start: Coordinate
    end: Coordinate
    cost_from_start: dict[tuple[Coordinate,Direction],int] | None

    def __init__(self, input_text: str) -> None:
        lines = [l.strip() for l in input_text.strip().split('\n')]
        self.width = len(lines[0])
        self.height = len(lines)
        assert self.width == self.height
        assert all([len(l) == self.width for l in lines])
        self.walls = set()
        self.cost_from_start = None

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

    def prettyprint(self, visited: Iterable[Coordinate] = frozenset()) -> str:
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

    def walk_dir(self, start: Coordinate, end: Coordinate, dir: Direction) -> dict[tuple[Coordinate,Direction], int]:
        startpos = MazePos(end.distance(start), 0, start, dir)
        working_set: queue.PriorityQueue = queue.PriorityQueue()
        working_set.put(startpos)

        least_cost_to_pos: dict[tuple[Coordinate,Direction], int] = {}
        all_cell_count = self.width * self.height - len(self.walls)
        iteration = 0

        while not working_set.empty():
            cur_dist, cur_cost, cur_pos, cur_dir = working_set.get()
            if iteration % 1000000 == 0:
                print(f'  # iteration {iteration}, queue size {working_set.qsize()}, visited cells {len(least_cost_to_pos)}/{all_cell_count}')
            iteration += 1

            if (cur_pos,cur_dir) in least_cost_to_pos and least_cost_to_pos[cur_pos,cur_dir] < cur_cost:
                continue  # current path+turning is longer than alternative, end of path
            if cur_pos == end:
                print(f"found a walk with cost {cur_cost}, remaining deque length is {working_set.qsize()}")
                continue  # else nothing, end of path

            # alternatives forward:
            for new_dir,new_cost in [
                (cur_dir, cur_cost + 1),
                (cur_dir.turn_left(), cur_cost + 1001),
                (cur_dir.turn_right(), cur_cost + 1001),
            ]:
                new_pos = cur_pos.step(new_dir)
                if (new_pos not in self.walls
                        and ((new_pos, new_dir) not in least_cost_to_pos
                             or least_cost_to_pos[(new_pos, new_dir)] > new_cost)
                ):
                    new_dist = self.end.distance(new_pos)
                    # special case: if we turned, then we also have to remember the cur_pos with turn
                    if new_dir != cur_dir:
                        least_cost_to_pos[cur_pos,new_dir] = cur_cost + 1000
                    least_cost_to_pos[new_pos,new_dir] = new_cost
                    working_set.put(MazePos(new_dist, new_cost, new_pos, new_dir))
        return least_cost_to_pos

    def part1(self) -> int:
        if self.cost_from_start is None:
            self.cost_from_start = self.walk_dir(self.start, self.end, Direction.E)
        return min(
            [c for c in [
                self.cost_from_start.get((self.end, d), None) for d in Direction
            ] if c]
        )

    def part2(self) -> int:
        # two end points to build alternative distance costs
        cost_table_end1 = self.walk_dir(self.end, self.start, Direction.W)
        cost_table_end2 = self.walk_dir(self.end, self.start, Direction.S)
        result = {self.start, self.end}

        # check if cost from start and cost from end combine for the best path length
        best_path_cost = self.part1()
        for y in range(self.height):
            for x in range(self.width):
                pos = Coordinate(x,y)
                if pos in self.walls: continue
                for dir in Direction:
                    cost_from_start = self.cost_from_start.get((pos, dir), -1)
                    cost_from_end1 = cost_table_end1.get((pos, dir.turn_back()), -1)
                    cost_from_end2 = cost_table_end2.get((pos, dir.turn_back()), -1)
                    if (cost_from_start >= 0
                        and ((cost_from_end1 >= 0 and cost_from_start + cost_from_end1 == best_path_cost)
                          or (cost_from_end2 >= 0 and cost_from_start + cost_from_end2 == best_path_cost))):
                        result.add(pos)  # ignore dir
        print(f"found {len(result)} best positions: {result}")
        return len(result)


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
    print(f"lowest cost: {m.part1()}")

    # part 2
    print(f"best tiles: {m.part2()}")
