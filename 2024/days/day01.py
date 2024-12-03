import itertools
from .helper import get_input_data

def solve():
    example = """
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3"""
    input_text = get_input_data(2024, 1, example)

    left = []
    right = []
    for pair in input_text.strip().splitlines():
        l,r = pair.split()
        left.append(int(l))
        right.append(int(r))
    left.sort()
    right.sort()

    # part 1
    print("sum: ", sum(itertools.starmap((lambda a,b: abs(a-b)), zip(left, right))))

    # part 2
    print("score: ", sum([item * right.count(item) for item in left]))
