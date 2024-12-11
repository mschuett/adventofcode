from .helper import get_input_data
from typing import List,Set,Dict
from functools import lru_cache
from collections import Counter

class Stones(object):
    """simple implementation"""
    stones: List[int]
    def __init__(self, input_text: str):
        self.stones = [int(i) for i in input_text.strip().split()]
    def __str__(self):
        return str(self.stones)
    def simple_blink(self):
        new_stones = []
        for s in self.stones:
            if s == 0:
                new_stones.append(1)
            elif len(str(s)) % 2 == 0:
                digits = len(str(s)) // 2
                new_stones.append(int(str(s)[:digits]))
                new_stones.append(int(str(s)[digits:]))
            else:
                new_stones.append(s*2024)
        self.stones = new_stones


class StonesPart2(Stones):
    """new implementation, only use dict of number counts"""
    stonedict: dict

    def __init__(self, input_text: str):
        super(StonesPart2, self).__init__(input_text)
        self.stonedict = Counter(self.stones)

    def stone_count(self):
        return sum(self.stonedict.values())

    def __str__(self):
        return f"{self.stonedict}\n{self.stone_count()} stones"

    @staticmethod
    @lru_cache(maxsize=4092)
    def compute_blinks(num: int, cycles: int = 5) -> List[int]:
        start = [num]
        result = []
        for i in range(cycles):
            result = []
            for num in start:
                if num == 0:
                    result.append(1)
                elif len(str(num)) % 2 == 0:
                    digits = len(str(num)) // 2
                    result.append(int(str(num)[:digits]))
                    result.append(int(str(num)[digits:]))
                else:
                    result.append(num * 2024)
            start = result
        return result

    def blink(self, cycles: int = 5):
        granularity = 5
        assert cycles % granularity == 0

        for cycle in range(cycles // granularity):
            cycle_dict = {}
            for num in self.stonedict:
                num_result = dict(Counter(self.compute_blinks(num, granularity)))
                for newnum in num_result:
                    if newnum not in cycle_dict:
                        cycle_dict[newnum] = num_result[newnum] * self.stonedict[num]
                    else:
                        cycle_dict[newnum] += num_result[newnum] * self.stonedict[num]
            self.stonedict = cycle_dict
            print(f"cycle {cycle+1} blink {(cycle+1) * granularity}: {self.stone_count()} stones")


def solve():
    example = """0 1 10 99 999"""
    example = """125 17"""
    input_text = get_input_data(2024, 11, example)

    # part 1
    s = Stones(input_text)
    print(s)
    for _ in range(25):
        s.simple_blink()
        # print(s)
    print(len(s.stones))

    # part 2
    s2 = StonesPart2(input_text)
    print(s2)
    s2.blink(75)
    # print(s2.compute_blinks.cache_info())
    print(f"{s2.stone_count()} stones")
