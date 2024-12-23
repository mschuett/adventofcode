from .helper import get_input_data,is_example

from typing import *
import textwrap
import collections

class Coordinate(object):
    x: int
    y: int
    def __init__(self,x,y):
        self.x = x
        self.y = y
    def __hash__(self):
        return hash((self.x, self.y))
    def __str__(self):
        return f"({self.x},{self.y})"
    def __repr__(self):
        return f"({self.x},{self.y})"
    def __eq__(self, other):
        return self.x == other.x and self.y == other.y
    def __lt__(self,other):
        return self.x < other.x and self.y < other.y
    def neighbours(self) -> list['Coordinate']:
        return [
            Coordinate(self.x, self.y - 1),
            Coordinate(self.x + 1, self.y),
            Coordinate(self.x, self.y + 1),
            Coordinate(self.x - 1, self.y),
        ]
    def distance(self,other: 'Coordinate') -> int:
        return abs(self.x - other.x) + abs(self.y - other.y)


class RamMap(object):
    width: int
    height: int
    start: Coordinate
    end: Coordinate
    time: int
    bytes_to_fail: list[Coordinate]
    failed_bytes: set[Coordinate]

    def __init__(self, input_text: str, size: int):
        self.width, self.height = size, size
        self.start = Coordinate(0, 0)
        self.end = Coordinate(size-1, size-1)
        self.time = 0
        self.bytes_to_fail = [Coordinate(int(x), int(y)) for x,y in [line.split(",") for line in input_text.strip().splitlines()]]
        self.failed_bytes = set()

    def __repr__(self) -> str:
        return f'{self.__class__.__name__}(width={self.width}, height={self.height}, time={self.time})'

    def __str__(self) -> str:
        return self.prettyprint()

    def prettyprint(self, visited: set[Coordinate] = frozenset()) -> str:
        outstr = self.__repr__() + '\n'
        for y in range(self.height):
            for x in range(self.width):
                pos = Coordinate(x,y)
                if pos in self.failed_bytes:
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

    def advance_time(self, nanoseconds: int) -> None:
        self.failed_bytes.update(self.bytes_to_fail[self.time:self.time+nanoseconds])
        self.time += nanoseconds

    def in_map(self, pos: Coordinate) -> bool:
        return 0 <= pos.x < self.width and 0 <= pos.y < self.height
    def is_free(self, pos: Coordinate) -> bool:
        return pos not in self.failed_bytes

    def walk(self) -> int | None:
        class WalkPos(NamedTuple):
            pos: Coordinate
            path: set[Coordinate]
            def cost(self) -> int:
                return len(self.path)

        working_set = collections.deque([WalkPos(self.start, set())])
        best_walk: WalkPos | None = None
        visited: dict[Coordinate,int] = { }
        iteration = 0

        while working_set:
            cur_pos, cur_path = working_set.pop()

            if iteration % 100000 == 0:
                print(f'  # iteration {iteration}, deque size {len(working_set)}, visited cells {len(visited)}')
            iteration += 1

            if cur_pos == self.end:
                if not (best_walk and best_walk.cost() < len(cur_path)):
                    best_walk = WalkPos(cur_pos, cur_path)
                    # print(f"found a walk with len {len(cur_path)}, remaining deque length is {len(working_set)}")
                continue  # else nothing, end of path
            elif best_walk and best_walk.cost() < len(cur_path):
                continue  # current path is longer than alternative, end of path
            elif best_walk and best_walk.cost() < len(cur_path) + cur_pos.distance(self.end):
                continue  # small optimization, we cannot reach the end faster than the old solution, end of path

            if cur_pos not in visited:
                visited[cur_pos] = len(cur_path)
            elif visited[cur_pos] > len(cur_path):
                visited[cur_pos] = len(cur_path)
            else:
                continue  # alternative route was shorter, end of path

            options = [pos for pos in cur_pos.neighbours()
                       if self.in_map(pos)
                       and self.is_free(pos)
                       and pos not in cur_path]
            for opt in options:
                working_set.append(WalkPos(opt, cur_path | {cur_pos}))
        if best_walk is None:
            return None
        print(f"time {self.time}, found path with cost {best_walk.cost()}")
        print(self.prettyprint(best_walk.path))
        return best_walk.cost()

    def find_first_impossible_walk(self) -> tuple[int,Coordinate]:
        # search backward in time, because the search space is very small if everything is blocked
        # => so we search the last time with any possible walk
        self.time = len(self.bytes_to_fail)

        while True:
            self.failed_bytes = set(self.bytes_to_fail[:self.time])
            result = self.walk()
            if result:  # one of, to return the first impossible walk
                return self.time+1, self.bytes_to_fail[self.time]
            else:
                self.time -= 1

def solve():
    example = """\
    5,4
    4,2
    4,5
    3,0
    2,1
    6,3
    2,4
    1,5
    0,6
    3,3
    2,6
    5,1
    1,2
    5,5
    2,5
    6,5
    1,4
    0,4
    6,4
    1,1
    6,1
    1,0
    0,5
    1,6
    2,0
    """
    input_text = textwrap.dedent(get_input_data(2024, 18, example))

    # part 1
    if is_example():
        rm = RamMap(input_text, 6+1)
        time = 12
    else:
        rm = RamMap(input_text, 70+1)
        time = 1024

    # part 1
    # really slow :-(
    rm.advance_time(time)
    print(rm.walk())

    # part 2
    # this is really fast
    time, pos = rm.find_first_impossible_walk()
    print(f"failure at {time}, last failed byte at {pos}")
