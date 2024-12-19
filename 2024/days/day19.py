from .helper import get_input_data

import textwrap
import functools

class TowelConstructor(object):
    towels: list[str]
    target_patterns: list[str]

    def __init__(self, input_data: str) -> None:
        tow, pat = input_data.strip().split('\n\n')
        self.towels = tow.split(', ')
        self.target_patterns = pat.strip().split('\n')

    def __repr__(self) -> str:
        return f'TowelConstructor({self.towels}, {self.target_patterns})'

    def recursive_search(self, pattern: str) -> int:
        towels = self.towels

        @functools.cache
        def recursive(target_pattern: str) -> int:
            if not target_pattern:
                return 1

            acc = 0
            # subtract from left and right
            for t_left in [t for t in towels if len(t) <= len(target_pattern) and target_pattern[:len(t)] == t]:
                    acc += recursive(target_pattern[len(t_left):])
            return acc
        return recursive(pattern)

        # def recursive_with_debug_output(target_pattern: str, found_left: list[str]) -> int:
        #     if not target_pattern:
        #         print("found " + str(found_left))
        #         return 1
        #
        #     acc = 0
        #     for t_left in [t for t in towels if len(t) <= len(target_pattern) and target_pattern[:len(t)] == t]:
        #             acc += recursive_with_output(target_pattern[len(t_left):], found_left+[t_left])
        #     return acc
        # return recursive_with_debug_output(pattern, [])

    def search(self) -> tuple[int,int]:
        acc_bool = 0
        acc_int = 0
        for i, pat in enumerate(self.target_patterns):
            res = self.recursive_search(pat)
            acc_int += res
            acc_bool += bool(res)
            print(f"# {i}/{len(self.target_patterns)}: {res} for pattern {pat}")
        return acc_bool, acc_int


def solve():
    example = """\
    r, wr, b, g, bwu, rb, gb, br
    
    brwrr
    bggr
    gbbr
    rrbgbr
    ubwu
    bwurrg
    brgr
    bbrgwb
    """
    input_text = textwrap.dedent(get_input_data(2024, 19, example))

    # part 1
    tc = TowelConstructor(input_text)
    print(tc)
    matches, combinations = tc.search()
    print(f"possible designs: {matches}")

    # part 2
    print(f"number of different ways: {combinations}")
