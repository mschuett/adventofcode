from .helper import get_input_data
from typing import List,Set,Dict


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

class TrailMap2d(object):
    lines: List[str]
    width: int
    height: int
    trailheads: Set[Coordinate]
    trails: Dict[Coordinate,List[List[Coordinate]]]

    def __init__(self, input_text):
        self.lines = [l.strip() for l in input_text.strip().split('\n')]
        self.width = len(self.lines[0])
        self.height = len(self.lines)
        assert self.width == self.height
        assert all([len(l) == self.width for l in self.lines])
        self.trailheads = set()
        self.trails = {}

        for y in range(self.height):
            for x in range(self.width):
                if self.lines[y][x] == '0':
                    self.trailheads.add(Coordinate(x,y))

    def pos_level(self, pos: Coordinate) -> int:
        return -1 if self.lines[pos.y][pos.x] == '.' else int(self.lines[pos.y][pos.x])

    def find_all_trails(self):
        for head in self.trailheads:
            self.trails[head] = self.find_trails_from_head(head, [head])

    def find_next_steps(self, head: Coordinate) -> List[Coordinate]:
        level = self.pos_level(head)
        neighbours = [
            Coordinate(head.x, head.y-1),
            Coordinate(head.x-1, head.y),
            Coordinate(head.x, head.y+1),
            Coordinate(head.x+1, head.y),
        ]
        result = [c for c in neighbours if (self.in_area(c) and (level + 1 == self.pos_level(c)))]
        # print(f"find_next_steps({head}, {level}): {[(c, self.pos_level(c)) for c in result]}")
        return result

    def find_trails_from_head(self, head: Coordinate, path: List[Coordinate]) -> List[List[Coordinate]]:
        if self.pos_level(head) == 9:
            return [path]
        next_steps = self.find_next_steps(head)
        if len(next_steps) == 0:
            return []
        if len(next_steps) == 1:
            step = next_steps[0]
            return self.find_trails_from_head(step, path + [step])
        # else: split path and search branches
        results = []
        for next_step in next_steps:
            new_path = path[:]
            new_path.append(next_step)
            path_result = self.find_trails_from_head(next_step, new_path)
            for result in path_result:
                results.append(result)
        return results

    def __str__(self):
        mapstr = ""
        for y in range(self.height):
            for x in range(self.width):
                if Coordinate(x,y) in self.trailheads:
                    mapstr += '0'
                else:
                    mapstr += "."
            mapstr += "\n"
        return f"{self.__class__.__name__}({self.width},{self.height})\n{mapstr}"

    def in_area(self, pos: Coordinate) -> bool:
        return (0 <= pos.x < self.width) and (0 <= pos.y < self.height)

    def sort_trails(self) -> dict:
        acc = {}
        for head in self.trailheads:
            for trail in self.trails[head]:
                if (trail[0],trail[-1]) in acc:
                    acc[(trail[0],trail[-1])] += 1
                else:
                    acc[(trail[0],trail[-1])] = 1
        return acc

    def score(self) -> int:
        acc = self.sort_trails()
        return len(acc.keys())

    def ratings(self) -> int:
        acc = self.sort_trails()
        return sum(acc.values())



def solve():
    example = """\
    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732"""
    input_text = get_input_data(2024, 10, example)

    m = TrailMap2d(input_text)
    m.find_all_trails()

    # part 1
    print(m.score())

    # part 2
    print(m.ratings())
