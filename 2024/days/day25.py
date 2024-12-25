import textwrap
from typing import *
from .helper import get_input_data


def read_input(input_text: str) -> tuple[list,list]:
    def rotate_lines_90deg(lines: list[str]) -> List[str]:
        return ["".join(l) for l in zip(*lines[::-1])]
    items = input_text.split('\n\n')
    keys = []
    locks = []
    for item in items:
        lines = item.splitlines()
        if lines[0] == "#####":
            lines = rotate_lines_90deg(lines[1:-1])
            lines = rotate_lines_90deg(lines)
            lines = rotate_lines_90deg(lines)
            lines.reverse()
            lock = [l.count('#') for l in lines]
            print(f"lock: {lines} -> {lock}")
            locks.append(lock)
        else:
            lines = rotate_lines_90deg(lines[1:-1])
            key = [l.count('#') for l in lines]
            print(f"key: {lines} -> {key}")
            keys.append(key)
    return keys, locks

def solve():
    example = """\
    #####
    .####
    .####
    .####
    .#.#.
    .#...
    .....
    
    #####
    ##.##
    .#.##
    ...##
    ...#.
    ...#.
    .....
    
    .....
    #....
    #....
    #...#
    #.#.#
    #.###
    #####
    
    .....
    .....
    #.#..
    ###..
    ###.#
    ###.#
    #####
    
    .....
    .....
    .....
    #....
    #.#..
    #.#.#
    #####
    """
    input_text = textwrap.dedent(get_input_data(2024, 25, example))

    # part 1
    keys, locks = read_input(input_text)
    max_height = 5
    matching_keys = 0
    for lock in locks:
        for key in keys:
            if all([a+b <= max_height for a, b in zip(key, lock)]):
                matching_keys += 1
    print(matching_keys)
