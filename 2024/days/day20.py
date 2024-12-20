import collections
import itertools
import textwrap
from typing import *
from .helper import get_input_data
from .day16 import ReindeerMaze, Coordinate


class ShortcutMap(ReindeerMaze):
    distmap: dict[Coordinate,int]

    def __init__(self, input_text: str):
        super().__init__(input_text)
        self.distmap = {}

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

    def walk(self):
        working_set = collections.deque()
        working_set.append((self.start, 0))
        visited = {self.start: 0}

        while working_set:
            pos, dist = working_set.pop()
            if pos == self.end:
                continue

            new_dist = dist+1
            for new_pos in pos.neighbours():
                if new_pos not in self.walls and (
                        new_pos not in visited or visited[new_pos] > new_dist):
                    visited[new_pos] = new_dist
                    working_set.append((new_pos, new_dist))
        return visited

    def find_short_shortcuts(self):
        if not self.distmap:
            self.distmap = self.walk()
        distmap = self.distmap
        racetime = distmap[self.end]
        cheats = {}
        for y in range(1, self.height-1):
            for x in range(1, self.width-1):
                pos = Coordinate(x,y)
                if pos not in self.walls: continue

                for pos1,pos2 in [
                    (Coordinate(x-1,y), Coordinate(x+1,y)),  # east/west
                    (Coordinate(x,y-1), Coordinate(x,y+1))   # north/south
                ]:
                    if (pos1 in distmap and pos2 in distmap
                            and distmap[pos1] <= racetime
                            and distmap[pos2] <= racetime):
                        saving = abs(distmap[pos1] - distmap[pos2]) - 2
                        cheats[saving] = cheats.get(saving, 0) + 1
        return cheats

    def part1(self, cutoff = 100) -> int:
        cheats = self.find_short_shortcuts()
        solution_count = sum([cheats[k] for k in cheats if k >= cutoff])
        return solution_count

    def find_long_shortcuts(self, maxlen = 20):
        distmap = self.walk()
        cheats = {}
        for (src_pos, dst_pos) in itertools.combinations(self.distmap.keys(), 2):
            dist = src_pos.distance(dst_pos)
            if dist > maxlen:
                continue  # too far away
            if abs(distmap[src_pos] - distmap[dst_pos]) == dist:
                continue  # normal path is already minimal
            # no need for any pathfinding, we can step across walls and tiles, so path = dist
            saving = abs(distmap[src_pos] - distmap[dst_pos]) - dist
            cheats[src_pos, dst_pos] = saving
        return cheats

    def part2(self, cutoff = 100) -> int:
        cheats = self.find_long_shortcuts()
        savings: dict[int,int] = {}
        for src_pos,dst_pos in cheats:
            savings[cheats[src_pos,dst_pos]] = savings.get(cheats[src_pos,dst_pos], 0) + 1

        for k in sorted(savings):
            if k >= cutoff:
                print(f"found {savings[k]} cheats saving {k} picoseconds")

        solution_count = sum([savings[k] for k in savings if k >= cutoff])
        return solution_count


def solve():
    example = """\
    ###############
    #...#...#.....#
    #.#.#.#.#.###.#
    #S#...#.#.#...#
    #######.#.#.###
    #######.#.#...#
    #######.#.###.#
    ###..E#...#...#
    ###.#######.###
    #...###...#...#
    #.#####.#.###.#
    #.#...#.#.#...#
    #.#.#.#.#.#.###
    #...#...#...###
    ###############
    """
    input_text = textwrap.dedent(get_input_data(2024, 20, example))

    # part 1
    m = ShortcutMap(input_text)
    print(f"result is {m.part1()}")

    # part 2
    print(f"result is {m.part2()}")
