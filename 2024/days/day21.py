import collections
import functools
import queue
import itertools
import textwrap
from enum import Enum
from typing import *
from .helper import get_input_data, is_example
from .day16 import Coordinate

class Direction(Enum):
    N = '^'
    E = '>'
    S = 'v'
    W = '<'
    def __str__(self):
        return self.value
    def __repr__(self):
        return self.value

@functools.cache
def score_movement(movement: tuple[Direction,...]) -> int:
    acc = 0
    dir_cost = {
        Direction.N: 1,
        Direction.E: 1,
        Direction.S: 2,
        Direction.W: 3,
    }
    for d in movement:
        acc += dir_cost[d]
    for a,b in itertools.pairwise(movement):
        if a != b: acc += 1
    return acc

@functools.cache
def path_to_movement(path: tuple[Coordinate,...]) -> List[Direction]:
    chunksize = 5
    result = []
    if len(path) <= chunksize:
        for a,b in itertools.pairwise(path):
            if a.x == b.x and a.y+1 == b.y: result.append(Direction.S)
            elif a.x == b.x and a.y-1 == b.y: result.append(Direction.N)
            elif a.x+1 == b.x and a.y == b.y: result.append(Direction.E)
            elif a.x-1 == b.x and a.y == b.y: result.append(Direction.W)
            else: raise ValueError("invalid movement")
        return result
    i = 0
    while i+chunksize <= len(path):
        chunk = path[i:i+chunksize]
        result.extend(path_to_movement(chunk))
        i += 1
    last_chunk = path[i:]
    result.extend(path_to_movement(last_chunk))
    return result

@functools.cache
def movement_to_string(movement: tuple[Direction,...]) -> str:
    return "".join([d.value for d in movement])

class Keypad(object):
    tiles: List[List[str|None]]
    height: int
    width: int
    def __init__(self, tiles: List[List[str|None]]) -> None:
        self.tiles = tiles
        self.height = len(tiles)
        self.width = len(tiles[0])

    @functools.cache
    def get_pos(self, needle: str) -> Coordinate|None :
        for y in range(len(self.tiles)):
            for x in range(len(self.tiles[y])):
                if self.tiles[y][x] == needle:
                    return Coordinate(x,y)
        return None

    @functools.cache
    def find_path(self, start: str, end: str) -> List[Coordinate]:
        start_pos = self.get_pos(start)
        end_pos   = self.get_pos(end)
        if not start_pos or not end_pos:
            raise ValueError("cannot get start/end position")
        distance = start_pos.distance(end_pos)
        if distance == 1:
            return [start_pos, end_pos]

        # otherwise: very short path dfs
        # one caveat: due to the layout of the direction keypad, different paths have different costs
        # repetitions are good, and ^> are closer to the A than v< thus better
        all_paths = []
        working_set: queue.PriorityQueue[tuple[int, Coordinate, list[Coordinate]]] = queue.PriorityQueue()
        working_set.put((end_pos.distance(start_pos), start_pos, [start_pos]))
        while not working_set.empty():
            cur_dist, cur_pos, cur_path = working_set.get()
            if cur_pos == end_pos:
                all_paths.append(cur_path)
            for next_pos in cur_pos.neighbours():
                if (0 <= next_pos.x < self.width and
                    0 <= next_pos.y < self.height and
                    self.tiles[next_pos.y][next_pos.x] is not None):
                    next_dist = end_pos.distance(next_pos)
                    if next_dist > cur_dist: continue  # skip if moving away
                    working_set.put((next_dist, next_pos, cur_path + [next_pos]))
        movements_with_score = []
        for path in all_paths:
            m = path_to_movement(tuple(path))
            score = score_movement(tuple(m))
            movements_with_score.append((score, path))
        movements_with_score.sort(key=lambda x: x[0])
        #print(f"find_path({start}, {end}): found movements:")
        # for item in movements_with_score: print(item)
        return movements_with_score[0][1]

    @functools.cache
    def generate_dir_keys(self, sequence: str) -> str:
        @functools.cache
        def generate_dir_keys_pair(src_pos: str, dst_pos: str) -> str:
                path = self.find_path(src_pos, dst_pos)
                mvmt = path_to_movement(tuple(path))
                text = movement_to_string(tuple(mvmt)) + "A"
                return text
        result = []
        for src_pos,dst_pos in itertools.pairwise("A" + sequence):
            result.append(generate_dir_keys_pair(src_pos, dst_pos))
        return "".join(result)

class NumericKeypad(Keypad):
    def __init__(self):
        tiles = [["7", "8", "9"],
                 ["4", "5", "6"],
                 ["1", "2", "3"],
                 [None, "0", "A"]]
        super().__init__(tiles)

class DirectionKeypad(Keypad):
    def __init__(self):
        tiles = [[None, "^", "A"],
                 ["<", "v", ">"]]
        super().__init__(tiles)

def solve_part1(codes: List[str], robot_keypads: int = 2) -> int:
    n = NumericKeypad()
    d = DirectionKeypad()
    acc = 0
    for code in codes:
        keys = n.generate_dir_keys(code)
        for _ in range(robot_keypads):
            keys = d.generate_dir_keys(keys)
        num = int(code.replace('A', ''))
        comp = num * len(keys)
        acc += comp
        counter = { k: v for k,v in (collections.Counter(keys.split('A')).items()) }
        print(f"{code} with complexity ({len(keys)}*{num}={comp}): {counter}")
    return acc

def solve_part2(codes: List[str], robot_keypads: int = 25) -> int:
    n = NumericKeypad()
    d = DirectionKeypad()
    acc = 0
    first_loop = sub_loop = (robot_keypads - 1) // 2
    assert first_loop + sub_loop +1 == robot_keypads
    for code in codes:
        keys = n.generate_dir_keys(code)
        keys = d.generate_dir_keys(keys)
        chunks = keys.split('A')
        keylength = 0
        for chunk in chunks:
            keys = chunk+'A'
            for _ in range(first_loop):
                keys = d.generate_dir_keys(keys)
            sub_chunks = keys.split('A')
            for sub_chunk in sub_chunks:
                keys = sub_chunk+'A'
                for _ in range(sub_loop):
                    keys = d.generate_dir_keys(keys)
                keylength += len(keys)
            keylength -= 1
        keylength -= 1
        num = int(code[:-1])
        comp = num * keylength
        acc += comp
        print(f"{code} with complexity ({keylength}*{num}={comp})")
    return acc


def solve():
    example = """\
    029A
    980A
    179A
    456A
    379A
    """
    input_text = textwrap.dedent(get_input_data(2024, 21, example))
    inputs = input_text.splitlines()

    # part 1
    comp_sum = solve_part1(inputs)
    print(f"sum of complexities: {comp_sum}")
    if is_example():
        assert comp_sum == 126384
    else:
        assert comp_sum == 152942

    # test if part1 and part2 functions return same results:
    # comp_sum1 = solve_part1(inputs, 9)
    # print(f"check: {comp_sum1}")
    # comp_sum2 = solve_part2(inputs, 9)
    # print(f"check: {comp_sum2}")
    # assert comp_sum1 == comp_sum2

    # part 2
    print(f"sum of complexities: {solve_part2(inputs, 25)}")
