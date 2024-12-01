import itertools
import os
import requests

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

day01()
