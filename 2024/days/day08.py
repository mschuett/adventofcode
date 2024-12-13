from .helper import get_input_data
from typing import List,Set,Dict
import itertools
import fractions

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
    # misusing the Coordinate to be a Vector as well
    def __sub__(self,other: 'Coordinate') -> 'Coordinate':
        return Coordinate(self.x-other.x, self.y-other.y)
    def __add__(self, other: 'Coordinate') -> 'Coordinate':
        return Coordinate(self.x+other.x, self.y+other.y)
    def __mul__(self, other: int):
        return Coordinate(self.x*other, self.y*other)
    def reduce_fraction(self) -> 'Coordinate':
        """observation: this was not necessary for my input"""
        f = fractions.Fraction(self.x, self.y)
        return Coordinate(f.numerator, f.denominator)

class AntennaMap2d(object):
    lines: List[str]
    width: int
    height: int
    antennaes: Dict[int, Set[Coordinate]]
    antinodes: Set[Coordinate]

    def __init__(self, input_text):
        lines = [l.strip() for l in input_text.strip().split('\n')]
        self.width = len(lines[0])
        self.height = len(lines)
        assert self.width == self.height
        assert all([len(l) == self.width for l in lines])
        self.antennaes = {}
        self.antinodes = set()

        for y in range(self.height):
            for x in range(self.width):
                if lines[y][x] == '.':
                    pass
                else:
                    antenna = ord(lines[y][x])
                    if not antenna in self.antennaes:
                        self.antennaes[antenna] = set()
                    self.antennaes[antenna].add(Coordinate(x,y))

    def __str__(self):
        antennaes = {}
        for atype in self.antennaes:
            for pos in self.antennaes[atype]:
                antennaes[pos] = chr(atype)
        mapstr = ""
        for y in range(self.height):
            for x in range(self.width):
                if Coordinate(x,y) in antennaes:
                    mapstr += antennaes[Coordinate(x,y)]
                elif Coordinate(x,y) in self.antinodes:
                    mapstr += "#"
                else:
                    mapstr += "."
            mapstr += "\n"
        return f"{self.__class__.__name__}({self.width},{self.height})\n{mapstr}"

    def in_area(self, pos: Coordinate) -> bool:
        return (0 <= pos.x < self.width) and (0 <= pos.y < self.height)

    def get_simple_antinodes(self):
        for atype in self.antennaes:
            for first,second in itertools.combinations(self.antennaes[atype], 2):
                v = first - second
                anti1 = second - v
                anti2 = first + v
                if self.in_area(anti1): self.antinodes.add(anti1)
                if self.in_area(anti2): self.antinodes.add(anti2)

    def get_resonance_antinodes(self):
        for atype in self.antennaes:
            for first,second in itertools.combinations(self.antennaes[atype], 2):
                self.antinodes.add(first)
                self.antinodes.add(second)
                v = (second - first).reduce_fraction()
                pos = second
                while True:
                    anti = pos - v
                    if not self.in_area(anti): break
                    self.antinodes.add(anti)
                    pos = anti
                pos = first
                while True:
                    anti = pos + v
                    if not self.in_area(anti): break
                    self.antinodes.add(anti)
                    pos = anti


def solve():
    example = """\
    ............
    ........0...
    .....0......
    .......0....
    ....0.......
    ......A.....
    ............
    ............
    ........A...
    .........A..
    ............
    ............
    """
    example2 = """\
    T.........
    ...T......
    .T........
    ..........
    ..........
    ..........
    ..........
    ..........
    ..........
    ..........
    """
    input_text = get_input_data(2024, 8, example2)

    # part 1
    m = AntennaMap2d(input_text)
    m.get_simple_antinodes()
    # print(m)
    print("unique locations: " + str(len(m.antinodes)))

    # part 2
    m = AntennaMap2d(input_text)
    m.get_resonance_antinodes()
    # print(m)
    print("unique resonance locations: " + str(len(m.antinodes)))
