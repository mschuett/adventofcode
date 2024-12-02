import itertools
from typing import *
from .helper import get_input_data

def solve():
    example = """\
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        """
    input_text = get_input_data(2024, 2, example)

    reports = []
    for line in input_text.strip().splitlines():
        reports.append([int(i) for i in line.split()])

    # part 1
    def is_safe(report: List[int]) -> bool:
        if report[0] > report[-1]:
            diffs = [(a-b) for a,b in itertools.pairwise(report)]
        else:
            diffs = [(b-a) for a,b in itertools.pairwise(report)]
        return all([(1 <= i <= 3) for i in diffs]) or all([(1 <= i <= 3) for i in reversed(diffs)])

    filtered = [is_safe(report) for report in reports]
    safe_count = sum(filtered)
    print("safe: ", safe_count)

    # part 2
    def is_safe_dampened(report: List[int]) -> bool:
        # brute force, test with each possible value removal
        for i in range(len(report)):
            report_dampened = report[:]
            del(report_dampened[i])
            if is_safe(report_dampened):
                return True
        return False

    filtered_dampened = [is_safe(report) or is_safe_dampened(report) for report in reports]
    safe_count_dampened = sum(filtered_dampened)
    print("safe dampened: ", safe_count_dampened)
