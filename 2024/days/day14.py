from .helper import get_input_data, is_example
import sys
from typing import *
from days.day08 import Coordinate

class Vector(Coordinate):
    pass

class Robot:
    pos: Coordinate
    vec: Vector

    def __init__(self, input_line: str) -> None:
        p,v = input_line.split(' ')
        p = p.split('=')[1]
        x,y = p.split(',')
        self.pos = Coordinate(int(x), int(y))
        v = v.split('=')[1]
        x,y = v.split(',')
        self.vec = Vector(int(x), int(y))
    def __repr__(self) -> str:
        return f'R(pos {self.pos}, vec {self.vec})'
    def step(self, map_width: int, map_height: int, steps: int = 1) -> None:
        new = self.pos + self.vec * steps
        self.pos.x = new.x % map_width
        self.pos.y = new.y % map_height


class RobotMap(object):
    width: int
    height: int
    robots: List[Robot]
    time: int

    def __init__(self, input_text: str, width: int, height: int) -> None:
        self.width, self.height = width, height
        lines = [l.strip() for l in input_text.strip().split('\n')]
        self.robots = [Robot(l) for l in lines]
        self.time = 0
    def __repr__(self) -> str:
        return f'RobotMap(seconds={self.time}, width={self.width}, height={self.height}): {self.robots}'
    def prettyprint(self) -> str:
        """pretty print showing robot count per field"""
        outlines = [self.__repr__()]
        for y in range(self.height):
            linestr = ""
            for x in range(self.width):
                count = sum([r.pos  == Coordinate(x,y) for r in self.robots])
                if count == 0:
                    linestr += '.'
                elif 0 < count <= 9:
                    linestr += str(count)
                else:
                    linestr += str(count-10)
            outlines.append(linestr)
        return "\n".join(outlines)
    def simpleprint(self) -> str:
        """faster print, showing robots without count"""
        outlines = [self.__repr__()]
        robo_positions = frozenset([r.pos for r in self.robots])
        for y in range(self.height):
            linestr = ""
            for x in range(self.width):
                if Coordinate(x,y) in robo_positions:
                    linestr += '#'
                else:
                    linestr += ' '
            outlines.append(linestr)
        return "\n".join(outlines)
    def __str__(self) -> str:
        return self.prettyprint()
    def map_step(self, seconds: int = 1) -> None:
        for r in self.robots:
            r.step(self.width, self.height, seconds)
        self.time += seconds
    def count_quadrants(self) -> List[int]:
        mid_width = self.width // 2
        mid_height = self.height // 2
        result = [0,0,0,0]
        for r in self.robots:
            if r.pos.x < mid_width and r.pos.y < mid_height:
                result[0] += 1
            elif r.pos.x > mid_width and r.pos.y < mid_height:
                result[1] += 1
            elif r.pos.x < mid_width and r.pos.y > mid_height:
                result[2] += 1
            elif r.pos.x > mid_width and r.pos.y > mid_height:
                result[3] += 1
            else:
                assert r.pos.x == mid_width or r.pos.y == mid_height
        return result

    def safety_factor(self) -> int:
        factors = self.count_quadrants()
        return factors[0]*factors[1]*factors[2]*factors[3]


def solve():
    example = """\
    p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3
    """
    input_text = get_input_data(2024, 14, example)

    # part 1
    if is_example():
        rm = RobotMap(input_text, 11,7)
    else:
        rm = RobotMap(input_text, 101,103)
    print(rm)
    rm.map_step(100)
    print(rm)
    print(rm.safety_factor())

    # part 2
    # had to look into reddit for this one. now checking if there is neat row of robots somewhere in the middle of the map

    # complicated but faster solution
    mid_ranges_x = [
        frozenset(range(20,30)),
        frozenset(range(30,40)),
        frozenset(range(40,50)),
        frozenset(range(50,60)),
        frozenset(range(60,70)),
    ]
    mid_range_len = len(mid_ranges_x[0])
    mid_range_y = frozenset(range(rm.height//2-10, rm.height//2+10))
    found_sth = False
    while not found_sth:
        if rm.time % 1000 == 0:
            print(f"time: {rm.time}")
        rm.map_step(1)
        for y in mid_range_y:
            row_pos = {r.pos.x for r in rm.robots if r.pos.y == y}
            if len(row_pos) >= mid_range_len and any([mid_range.issubset(row_pos) for mid_range in mid_ranges_x]):
                found_sth = True
                break
    print(rm)
    print(f"found at second {rm.time}")

    # simple but slow solution:
    while True:
        if "########" in rm.simpleprint():
            break
        rm.map_step(1)
    print(rm.simpleprint())
    print(f"found at second {rm.time}")
