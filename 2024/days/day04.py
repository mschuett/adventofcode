from .helper import get_input_data
from typing import List
import re

class TextMap2d(object):
    lines: List[str]
    width: int
    height: int
    def __init__(self, input_text):
        self.lines = [l.strip() for l in input_text.strip().split('\n')]
        self.width = len(self.lines[0])
        self.height = len(self.lines)
        assert self.width == self.height
        assert all([len(l) == self.width for l in self.lines])

    def __str__(self):
        return f"{self.__class__.__name__}({self.width},{self.height})\n"+'\n'.join(self.lines)

    def rotate_lines_90deg(self) -> List[str]:
        return ["".join(l) for l in zip(*self.lines[::-1])]

    def rotate_lines_45deg(self) -> List[str]:
        result_dict = {}
        for x in range(self.width):
            for y in range(self.height):
                i = x + y
                line = result_dict.get(i, "")
                line += self.lines[y][x]
                result_dict[i] = line
        result = []
        for i in sorted(result_dict.keys()):
            result.append(result_dict[i])
        return result

    def count_xmas_text(self, needle) -> int:
        """count text horizontally and diagonally"""
        count = 0
        rev_needle = needle[::-1]
        # simple case: right/left direction
        for l in self.lines:
            count += len(re.findall(needle, l))
            count += len(re.findall(rev_needle, l))
        # next case: up/down direction
        for l in self.rotate_lines_45deg():
            count += len(re.findall(needle, str(l)))
            count += len(re.findall(rev_needle, str(l)))
        return count

    def count_xmas_part2(self) -> int:
        """do not transform anything, but do stupid low-level comparisons"""
        count = 0
        for x in range(1, self.width-1):
            for y in range(1, self.height-1):
                if (self.lines[y][x] == 'A' and
                    # climbing diagonal
                    (
                        (self.lines[y+1][x-1] == 'M' and self.lines[y-1][x+1] == 'S') or
                        (self.lines[y+1][x-1] == 'S' and self.lines[y-1][x+1] == 'M')
                    ) and
                    # downward diagonal
                    (
                        (self.lines[y-1][x-1] == 'M' and self.lines[y+1][x+1] == 'S') or
                        (self.lines[y-1][x-1] == 'S' and self.lines[y+1][x+1] == 'M')
                    )):
                    count += 1
        return count


def solve():
    example = """\
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX"""
    input_text = get_input_data(2024, 4, example)

    # part 1
    # overly complex with array rotations
    m = TextMap2d(input_text)
    result = m.count_xmas_text("XMAS")
    m2 = TextMap2d('\n'.join(m.rotate_lines_90deg()))
    result += m2.count_xmas_text("XMAS")

    print("XMAS count: ", result)

    # part 2
    print("X-MAS count: ", m.count_xmas_part2())
