import itertools
import os
import requests
from typing import *

# session cookie after authentication/login
advent_headers = {
    'Cookie': 'session='+os.environ['ADVENT_AUTH_SESSION_ID']
}

def day01(real_data=False):
    if real_data:
        input_text = requests.get('https://adventofcode.com/2024/day/1/input', headers=advent_headers).text
    else:
        input_text = """
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3"""

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
    similarity_score = 0
    for item in left:
        similarity_score += item * right.count(item)
    print("score: ", similarity_score)

def day02(real_data=False):
    if real_data:
        input_text = requests.get('https://adventofcode.com/2024/day/2/input', headers=advent_headers).text
    else:
        input_text = """\
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        """

    reports = []
    for line in input_text.strip().splitlines():
        reports.append([int(i) for i in line.split()])
    print(reports)

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

day02()
