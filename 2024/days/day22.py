import itertools
import textwrap
from typing import *
from .helper import get_input_data


def mix(secret: int, value: int) -> int:
    return secret ^ value

def prune(secret: int) -> int:
    return secret % 16777216

def trade(secret: int) -> int:
    state = secret
    while True:
        state = prune(mix(state, state * 64))
        state = prune(mix(state, state // 32))
        state = prune(mix(state, state * 2048))
        yield state

def part1(inputs: Iterable[int]) -> int:
    cycles = 2000
    gen = [trade(i) for i in inputs]
    states = []
    state = 0
    for t in gen:
        for _ in range(cycles):
            state = next(t)
        states.append(state)
    # print(states)
    return sum(states)

def part2(inputs: list[int]) -> int:
    cycles = 2000
    seq_sums: dict[tuple,int] = {}

    for i in inputs:
        gen = trade(i)
        cur_prices = [i % 10] + [next(gen) % 10 for i in range(cycles)]
        cur_diffs = [b-a for a,b in itertools.pairwise(cur_prices)]
        seq_prices = {}
        for j in range(len(cur_diffs)-4):
            seq = tuple(cur_diffs[j:j+4])
            if seq not in seq_prices:
                seq_prices[seq] = cur_prices[j+4]
        for seq in seq_prices:
            seq_sums[seq] = seq_sums.get(seq, 0) + seq_prices[seq]

    max_price = max(seq_sums.values())
    return max_price

def solve():
    example = """\
    1
    10
    100
    2024
    """
    input_text = textwrap.dedent(get_input_data(2024, 22, example))
    inputs = [int(line) for line in input_text.splitlines()]

    # part 1
    print(part1(inputs))
    # part 2
    print(part2(inputs))
