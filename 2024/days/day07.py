from .helper import get_input_data
from typing import List
from enum import Enum
import itertools

class Op(Enum):
    plus = "+"
    mult = "*"
    conc = "||"  # part 2

class Equation(object):
    result: int
    numbers: List[int]
    operators: List[Op]

    def __init__(self, line: str):
        left, right = line.split(':')
        self.result = int(left)
        self.numbers = [int(n) for n in right.split()]
        self.operators = []

    def __repr__(self):
        return f"{self.result}: {self.numbers}, {self.operators}"

    def is_valid_with_ops(self, ops: List[Op]) -> bool:
        assert len(ops) == (len(self.numbers) - 1)
        acc = self.numbers[0]
        for i in range(len(ops)):
            if ops[i] == Op.plus:
                acc += self.numbers[i+1]
            elif ops[i] == Op.mult:
                acc *= self.numbers[i+1]
            elif ops[i] == Op.conc:
                acc = int("".join([str(acc), str(self.numbers[i+1])]))
            else:
                raise ValueError("Invalid operator")
            # fail fast
            if acc > self.result: return False
        return acc == self.result

    def find_ops_combination(self, ops: List[Op]) -> bool:
        ops_count = len(self.numbers) - 1

        for p in itertools.product(ops, repeat=ops_count):
            if self.is_valid_with_ops(list(p)):
                self.operators = list(p)
                return True
        return False


def solve():
    example = """\
    190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20
    """
    input_text = get_input_data(2024, 7, example)
    equations: List[Equation] = [Equation(l) for l in input_text.strip().splitlines()]

    # part 1
    for e in equations:
        e.find_ops_combination([Op.mult, Op.plus])
        # print(e)
    print("total calibration result: " + str(sum([e.result for e in equations if e.operators])))

    # part 2
    for e in equations:
        if not e.operators:
            e.find_ops_combination([Op.mult, Op.plus, Op.conc])
        # print(e)
    print("total calibration result: " + str(sum([e.result for e in equations if e.operators])))
