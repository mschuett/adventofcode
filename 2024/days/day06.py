from .helper import get_input_data
from typing import List,Tuple,Set
from enum import Enum,auto

class Direction(Enum):
    N = auto()
    E = auto()
    S = auto()
    W = auto()
    def __str__(self):
        return self.name
    def __repr__(self):
        return self.name
    def turn_right(self) -> 'Direction':
        if self == Direction.N:
            return Direction.E
        elif self == Direction.E:
            return Direction.S
        elif self == Direction.S:
            return Direction.W
        elif self == Direction.W:
            return Direction.N

class Coordinate(object):
    x: int
    y: int
    def __init__(self,x,y):
        self.x = x
        self.y = y
    def __eq__(self, other):
        return self.x == other.x and self.y == other.y
    def __hash__(self):
        return hash((self.x, self.y))
    def __str__(self):
        return f"({self.x},{self.y})"
    def __repr__(self):
        return f"({self.x},{self.y})"
    def step(self, d: Direction) -> 'Coordinate':
        if d == Direction.N:
            return Coordinate(self.x, self.y - 1)
        elif d == Direction.E:
            return Coordinate(self.x + 1, self.y)
        elif d == Direction.S:
            return Coordinate(self.x, self.y + 1)
        elif d == Direction.W:
            return Coordinate(self.x - 1, self.y)


class ObstacleMap2d(object):
    lines: List[str]
    width: int
    height: int
    obstacles: Set[Coordinate]
    start_position: Coordinate

    def __init__(self, input_text):
        lines = [l.strip() for l in input_text.strip().split('\n')]
        self.width = len(lines[0])
        self.height = len(lines)
        assert self.width == self.height
        assert all([len(l) == self.width for l in lines])
        self.obstacles = set()

        for y in range(self.height):
            for x in range(self.width):
                if lines[y][x] == '.':
                    pass
                elif lines[y][x] == '#':
                    self.obstacles.add(Coordinate(x,y))
                elif lines[y][x] == '^':
                    self.start_position = Coordinate(x,y)
                else:
                    raise ValueError('unexpected character in map')
        assert self.start_position, "found no start position"

    def __str__(self):
        return f"{self.__class__.__name__}({self.width},{self.height})"


class Walker(object):
    pos: Coordinate
    dir: Direction
    path: List[Tuple[Coordinate,Direction]]
    path_positions: Set[Coordinate]
    map: ObstacleMap2d

    def __init__(self, pos:Coordinate, dir: Direction, map: ObstacleMap2d) -> None:
        self.pos = pos
        self.dir = dir
        self.path = []
        self.path_positions = set()
        self.map = map

    def __str__(self):
        mapstr = ""
        for y in range(self.map.height):
            for x in range(self.map.width):
                if Coordinate(x,y) in self.map.obstacles:
                    mapstr += "#"
                elif Coordinate(x,y) == self.pos:
                    mapstr += self.dir.name
                elif Coordinate(x,y) in self.path_positions:
                    mapstr += "X"
                else:
                    mapstr += "."
            mapstr += "\n"
        return f"{self.__class__.__name__}({self.pos}, {self.dir}, {len(self.path)} steps over {len(self.path_positions)} positions: {self.path_positions}\n{mapstr}"

    def step(self):
        self.path.append((self.pos, self.dir))
        self.path_positions.add(self.pos)
        new_pos = self.pos.step(self.dir)
        while new_pos in self.map.obstacles:
            self.dir = self.dir.turn_right()
            new_pos = self.pos.step(self.dir)
        self.pos = new_pos

    def is_in_area(self) -> bool:
        return 0 <= self.pos.y < self.map.height and 0 <= self.pos.x < self.map.width

    def is_in_loop(self) -> bool:
        return (self.pos in self.path_positions) and (self.pos, self.dir) in self.path

    def walk(self):
        """walk until we move outside of the area or detect a loop"""
        while self.is_in_area() and not self.is_in_loop():
            self.step()


def solve():
    example = """\
    ....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#..."""
    input_text = get_input_data(2024, 6, example)

    m = ObstacleMap2d(input_text)

    # part 1
    w = Walker(m.start_position, Direction.N, m)
    w.walk()
    assert not w.is_in_area()
    assert not w.is_in_loop()
    print(w)
    print("distinct positions inside area: " + str(len(w.path_positions)))

    # part 2
    possible_obstructions = []
    for i,pos in enumerate(w.path_positions):
        print(f"check {i+1}/{len(w.path_positions)}: {pos}...")
        tmp_map = ObstacleMap2d(input_text)
        tmp_map.obstacles.add(pos)
        tmp_walk = Walker(tmp_map.start_position, Direction.N, tmp_map)
        tmp_walk.walk()
        if tmp_walk.is_in_loop() and tmp_walk.is_in_area():
            possible_obstructions.append(pos)
    print(f"{len(possible_obstructions)} possible positions: {possible_obstructions}")
