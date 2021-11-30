#!python3

import itertools
import os
import requests
from typing import *

# session cookie after authentication/login
advent_headers = {
  'Cookie': 'session='+os.environ['ADVENT_AUTH_SESSION_ID']
}


def day1():
    print("\nDay 1, Part 1")
    numbers = [1946, 1859, 1654, 1806, 1648, 1873, 1216, 1831, 1610, 1779, 1626, 1332, 1713, 1919, 1353, 1720, 1818, 1976, 1993, 1617, 1678, 1655, 1725, 1686, 1737, 1696, 1046, 1814, 1909, 1618, 2006, 1903, 1528, 1635, 1457, 1924, 1734, 1723, 1735, 1984, 1846, 1921, 1587, 2009, 1607, 1987, 1910, 1571, 1898, 1869, 1537, 1446, 1535, 1802, 1847, 1966, 1944, 1793, 1383, 1850, 1274, 347, 1208, 1748, 1906, 1771, 1849, 1773, 1792, 1705, 1538, 1564, 2003, 1994, 1545, 1704, 1657, 1483, 1701, 1724, 1293, 1834, 1712, 1950, 1844, 1290, 1692, 1820, 1585, 1986, 1328, 1841, 1709, 1232, 1945, 1684, 1787, 1991, 1914, 16, 1977, 1620, 1825, 1866, 1615, 1832, 496, 1932, 1819, 1559, 1870, 1677, 1650, 1594, 1664, 1600, 1622, 1862, 1937, 1624, 1580, 1931, 1803, 1839, 1755, 1952, 1473, 1694, 1864, 1178, 1163, 1790, 393, 1776, 1871, 1999, 1923, 1174, 1557, 1646, 1200, 1842, 1432, 1573, 1913, 1954, 1599, 1980, 1948, 1430, 1298, 1835, 1643, 1742, 1609, 1649, 1382, 1343, 1263, 1908, 1703, 1922, 1764, 1603, 1330, 588, 954, 1772, 1553, 975, 1499, 1552, 1214, 1829, 1698, 1797, 1807, 1961, 1947, 1845, 1881, 1821, 1815, 1623, 1675, 1478, 1886, 1951, 1700, 1890, 1876, 1781, 1853, 1983, 1901, 1939, 1292, 853, 1879, 1652]
    for x,y in [(x,y) for x,y in itertools.combinations(numbers, 2) if x+y == 2020]:
        print(f"X: {x}, Y: {y}, product: {x*y}")

    print("\nDay 1, Part 2")
    for x,y,z in [(x,y,z) for x,y,z in itertools.combinations(numbers, 3) if x+y+z == 2020]:
        print(f"X: {x}, Y: {y}, Z: {z}, product: {x*y*z}")


def day2():
    print("\nDay 2, Part 1")
    day2_test = '''\
    1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc
    '''
    day2_input = requests.get('https://adventofcode.com/2020/day/2/input', headers=advent_headers).text

    total = 0
    valid = 0
    for input_line in day2_input.splitlines():
        try:
            char_range, char, password = input_line.split()
            char_min, char_max = char_range.split('-')
            char = char[0]
        except Exception as e:
            print(f"Unexpected input:\n{input_line}\n raised {e}")
            continue
        count = password.count(char)
        is_valid = (int(char_min) <= count <= int(char_max))
        total += 1
        valid += is_valid
        # print('X' if is_valid else ' ', count, input_line)
    print(f"{valid} from {total} passwords are valid in 1st schema")

    print("\nDay 2, Part 2")
    total = 0
    valid = 0
    for input_line in day2_input.splitlines():
        try:
            char_range, char, password = input_line.split()
            char_min, char_max = char_range.split('-')
            char = char[0]
        except Exception as e:
            print(f"Unexpected input:\n{input_line}\n raised {e}")
            continue

        one = (password[int(char_min)-1] is char)
        two = (password[int(char_max)-1] is char)
        is_valid = ((one or two) and not (one and two))
        total += 1
        valid += is_valid
        # print('X' if is_valid else ' ', input_line)
    print(f"{valid} from {total} passwords are valid in 2nd schema")


def day3():
    forest = '''\
    ..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#
    '''
    forest = requests.get('https://adventofcode.com/2020/day/3/input', headers=advent_headers).text

    lines = forest.splitlines()
    height = len(lines)
    results = 1
    for down_slope,right_slope in [(1,1), (1,3), (1,5), (1,7), (2,1)]:
        y, x, tree_hits = 0, 0, 0
        while y < height:
            line = lines[y]
            hit = bool(line[x] == '#')
            tree_hits += hit
            # print(y, x, int(hit), line)
            x = (x + right_slope) % len(line)
            y = y + down_slope
        print(f"slope ({down_slope}, {right_slope}) hit {tree_hits} trees")
        results *= tree_hits

    print(f"multiplied results: {results}")


def day4():
    data = '''\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
'''
    data = requests.get('https://adventofcode.com/2020/day/4/input', headers=advent_headers).text

    # part 1
    required_categories = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'}
    valid_entries = 0
    total_entries = 0
    for entry in data.split("\n\n"):
        total_entries += 1
        fields = {}
        cleaned_entry = entry.replace("\n", " ")
        for field in cleaned_entry.split():
            key, value = field.split(":")
            fields[key] = value
        missing = required_categories - set(fields.keys())
        if missing:
            # print(f"missing fields {missing} in entry '{cleaned_entry}'")
            continue
        valid_entries += 1
    print(f"valid: {valid_entries} out of {total_entries}")

    import re
    hcl_re = re.compile('^#[0-9a-f]{6}$')
    hgt_re = re.compile('^([0-9]+)(cm|in)$')

    def byr(value):
        return value.isdigit() and (len(value) == 4) and (1920 <= int(value) <= 2002)

    def iyr(value):
        return value.isdigit() and (len(value) == 4) and (2010 <= int(value) <= 2020)

    def eyr(value):
        return value.isdigit() and (len(value) == 4) and (2020 <= int(value) <= 2030)

    def hgt(value):
        match = re.fullmatch(hgt_re, value)
        if not match:
            return False
        number = match.group(1)
        unit = match.group(2)
        if unit == 'in':
            return 59 <= int(number) <= 76
        elif unit == 'cm':
            return 150 <= int(number) <= 193
        else:
            return False

    def hcl(value):
        return hcl_re.fullmatch(value)

    def ecl(value):
        valid = ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
        return value in valid

    def pid(value):
        return value.isdigit() and (len(value) == 9)

    # part 2
    valid_entries = 0
    total_entries = 0
    for entry in data.split("\n\n"):
        total_entries += 1
        fields = {}
        check_failed = 0
        cleaned_entry = entry.replace("\n", " ")
        for field in cleaned_entry.split():
            key, value = field.split(":")
            fields[key] = value
        missing = required_categories - set(fields.keys())
        if missing:
            print(f"missing fields {missing} in entry '{cleaned_entry}'")
            continue
        for fieldname in required_categories:
            if not locals()[fieldname](fields[fieldname]):
                print(f"invalid {fieldname}: {fields[fieldname]}")
                check_failed = 1
                break
        if check_failed:
            continue
        print(f"valid entry: {fields}")
        valid_entries += 1
    print(f"valid: {valid_entries} out of {total_entries}")


def day5():
    data = [
        "BFFFBBFRRR",
        "FFFBBBFRRR",
        "BBFFBBFRLL",
        "FBFBBFFRLR"]
    data = requests.get('https://adventofcode.com/2020/day/5/input', headers=advent_headers).text

    max_seat = 0
    all_seats = []
    for code in data.splitlines():
        row_bin = code[0:7].translate({ord("F"): "0", ord("B"): "1"})
        col_bin = code[7:10].translate({ord("L"): "0", ord("R"): "1"})
        row = int(row_bin, 2)
        col = int(col_bin, 2)
        seat = row*8 + col
        max_seat = max(max_seat, seat)
        all_seats.append(seat)
        # print(f"row: {row_bin}={row}, column: {col_bin}={col}, seat: {seat}")
    all_seats.sort()
    print(f"min seat ID: {all_seats[0]}, max seat ID: {all_seats[-1]}")

    i = 1
    while i < len(all_seats) and all_seats[i] == all_seats[i-1] + 1:
        i += 1
    print(f"stopped at i={i}, between seat IDs {all_seats[i-1]} and {all_seats[i]} ==> your seat ID is {all_seats[i-1] + 1}")


def day6():
    data = '''\
abc

a
b
c

ab
ac

a
a
a
a

b
'''
    data = requests.get('https://adventofcode.com/2020/day/6/input', headers=advent_headers).text
    # part 1
    sum_of_answers = 0
    for group in data.split("\n\n"):
        answers = sorted(set(group.replace("\n", "")))
        count = len(answers)
        sum_of_answers += count
        print(f"count: {count}, group: {answers}")
    print(f"sum of answers: {sum_of_answers}")

    # part 2
    import string
    sum_of_everyone = 0
    for group in data.split("\n\n"):
        group_answer = set(string.ascii_lowercase)
        for person in group.strip().split("\n"):
            # print(f"person: {person} --> {set(person)}")
            group_answer &= set(person)
        count = len(group_answer)
        sum_of_everyone += count
        print(f"count: {count}, group: {sorted(group_answer)}")
    print(f"sum of everyone answers: {sum_of_everyone}")


def day7():
    data = '''\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
'''
    data = requests.get('https://adventofcode.com/2020/day/7/input', headers=advent_headers).text

    rules = {}
    for line in data.splitlines():
        outer, content = line.rstrip('.').split(' contain ')
        outer = outer.replace(" bags", "").replace(" bag", "")
        inner = {}
        if content == "no other bags":
            pass
        else:
            for item in content.split(", "):
                text = item.replace(" bags", "").replace(" bag", "")
                print(f"found item: {text}")
                count, colour = text.split(" ", 1)
                inner[colour] = int(count)
        rules[outer] = inner
    print(rules)

    # Part 1
    needle = 'shiny gold'
    containers = set()
    changed = True
    # init with needle
    for key, value in rules.items():
        if needle in value.keys():
            containers |= {key}

    # print(f"init: {containers}")
    # expand search until fixpoint
    while changed:
        changed = False
        new = []
        for needle in containers:
            for key, value in rules.items():
                if needle in value.keys():
                    new.append(key)
        if set(new) <= containers:
            pass
        else:
            containers |= set(new)
            changed = True
        # print(f"searching: {containers}")
    print(f"final: {int(len(containers))}")


    # Part 2
    needle = 'shiny gold'
    print(f"=== Part 2\n\ninit: {needle} ==> {rules[needle]}")

    def sub_count(needle):
        count = 1
        for key, value in rules[needle].items():
            subcount = sub_count(key)
            count += value * subcount
        print(f"sub_count({needle}) : {count}")
        return count

    all_count = sub_count(needle)
    print(f"final: {all_count-1}")


def day8():
    data = '''\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
'''
    data = requests.get('https://adventofcode.com/2020/day/8/input', headers=advent_headers).text

    # Part 1
    lines = data.splitlines()
    ic = 0
    acc = 0
    loop = False
    trace = []
    while not loop:
        trace.append(ic)
        op,arg_str = lines[ic].strip().split()
        arg = int(arg_str)
        if op == 'nop':
            ic += 1
        elif op == 'acc':
            acc += arg
            ic += 1
        elif op == 'jmp':
            ic += arg
        if ic in trace:
            loop = True
    print(f"final ic {ic}, acc {acc}")

    # Part 2
    # brute force: try to interchange all nop/jmp ops
    found_it = False
    for i in range(len(lines)):
        # print(f"check iteration {i}")
        ic = 0
        acc = 0
        loop = False
        abort = False
        trace = []
        while not loop and not abort:
            trace.append(ic)
            op,arg_str = lines[ic].strip().split()
            arg = int(arg_str)
            if i == ic: # interchange
                if op == 'acc':
                    abort = True
                    break
                elif op == 'nop':
                    op = 'jmp'
                elif op == 'jmp':
                    op = 'nop'
            if op == 'nop':
                ic += 1
            elif op == 'acc':
                acc += arg
                ic += 1
            elif op == 'jmp':
                ic += arg
            if ic in trace:
                loop = True
                abort = True
            elif ic == len(lines):
                print("reached the end")
                found_it = True
                abort = True
        if found_it:
            print(f"looped: {loop}, final ic {ic}, acc {acc}")
            break


def day9():
    data = '''\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
'''
    pre_length = 5
    data = requests.get('https://adventofcode.com/2020/day/9/input', headers=advent_headers).text
    pre_length = 25

    # Part 1
    def is_sum(n, array):
        from itertools import combinations
        for i,j in combinations(array, 2):
            if n == i+j:
                return True
        return False

    numbers = [int(i.strip()) for i in data.splitlines()]

    for i in range(pre_length, len(numbers)):
        if not is_sum(numbers[i], numbers[i-pre_length:i]):
            print(f"numbers[{i}] = {numbers[i]} is not a valid sum")
            pivot_number = numbers[i]
            break

    # Part 2
    for j in range(len(numbers)):
        for i in range(j):
            subset = numbers[i:j]
            if sum(subset) == pivot_number:
                print(f"found contiguous set {subset}, weakness is {min(subset)} + {max(subset)} = {min(subset) + max(subset)}")
                return


def day10():
    data = '''\
16
10
15
5
1
11
7
19
6
12
4
'''
    data = '''\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
'''
    data = requests.get('https://adventofcode.com/2020/day/10/input', headers=advent_headers).text
    sdata : List[int] = sorted([int(x) for x in data.splitlines()])
    diffs : List[int] = [0,0,0,0]
    last : int = 0
    for v in sdata:
        diffs[v - last] += 1
        last = v
    diffs[3] += 1  # for own device
    print(f"diffs: {diffs}, build-in: {sdata[-1] + 3}, result: {diffs[1] * diffs[3]}")

    def check_trace(trace, target) -> bool:
        if trace[-1] != target:
            return False
        last : int = 0
        for i in trace:
            if i > last + 3:
                return False
            last = i
        return True

    def find_paths(trace, items, target) -> List[List[int]]:
        # print(f"find_paths({trace}, {items}, {target})")
        results : List[List] = []
        last: int = trace[-1]
        if last == target:
            results.append(trace)
        else:
            for n in items:
                if last <= n <= last + 3:
                    new_items = [i for i in items if n <= i]
                    new_items.remove(n)
                    new_trace = find_paths(trace + [n], new_items, target)
                    if new_trace:
                        results += new_trace
        print(f"find_paths({trace}, {items}, {target}) --> {results}")
        return results

    def count_paths(trace, items, target) -> int:
        # print(f"count_paths({trace}, {items}, {target})")
        result : int = 0
        last: int = trace[-1]
        if last == target:
            return 1
        else:
            for n in items:
                if last <= n <= last + 3:
                    new_items = [i for i in items if n <= i]
                    new_items.remove(n)
                    result += count_paths(trace + [n], new_items, target)
        # print(f"count_paths({trace}, {items}, {target}) --> {result}")
        return result
    # print("count: ", count_paths([0], sdata, max(sdata)))

    result_set = set()

    def gen_traces(init, target, result_set) -> int:
        count = 1
        for i in init:
            copy = init.copy()
            copy.remove(i)
            if tuple(copy) not in result_set and check_trace(copy, target):
                #print(copy)
                result_set.add(tuple(copy))
                count += gen_traces(copy, target, result_set)
        return count

    print(gen_traces(sdata, sdata[-1], result_set))


def day11():
    data = '''\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
'''
    data = requests.get('https://adventofcode.com/2020/day/11/input', headers=advent_headers).text

    # Part 1
    def init(data):
        state = [[cell for cell in line.strip()] for line in data.splitlines()]
        size = len(state[0]), len(state)
        print(f"initial state with size {size}:")
        for line in state:
            print("".join(line))
        return state, size

    def occupied_neighbours(state, size, x, y):
        x_, y_ = size
        neighbours = [
            state[y-1][x-1] if y>0    and x>0 else '' , state[y-1][x] if y>0    else '', state[y-1][x+1] if y>0    and x<x_-1 else '',
            state[y  ][x-1] if            x>0 else '' ,                                  state[y  ][x+1] if            x<x_-1 else '',
            state[y+1][x-1] if y<y_-1 and x>0 else '' , state[y+1][x] if y<y_-1 else '', state[y+1][x+1] if y<y_-1 and x<x_-1 else '',
        ]
        return neighbours.count('#')

    def generation(state, size):
        x_, y_ = size
        state2 = [['?' for i in range(x_)] for i in range(y_)]
        changed = False

        for x,y in itertools.product(range(x_), range(y_)):
            state2[y][x] = state[y][x]
            if state[y][x] == 'L' and occupied_neighbours(state, size, x, y) == 0:
                state2[y][x] = '#'
                changed = True
            if state[y][x] == '#' and occupied_neighbours(state, size, x, y) >= 4:
                state2[y][x] = 'L'
                changed = True
        return state2, changed

    print("\nDay 11, Part 1")
    state, size = init(data)
    changed = True
    gen_count = 0
    while changed:
        gen_count += 1
        (state,changed) = generation(state, size)
        # print(f"\n\ngeneration {gen_count}:")
        # for line in state:
        #     print("".join(line))
    chars = "".join(["".join(line) for line in state])
    print(f"finally occupied chairs: {chars.count('#')}")

    # Part 2
    def seen_neighbours(state, size, x, y):
        x_, y_ = size
        neighbours = 0
        # check these directions in turn
        # 123
        # 4*5
        # 678

        # 2
        for y1 in range(y-1,-1,-1):
            if state[y1][x] == 'L':
                break
            if state[y1][x] == '#':
                neighbours += 1
                break
        # 7
        for y1 in range(y+1,y_):
            if state[y1][x] == 'L':
                break
            if state[y1][x] == '#':
                neighbours += 1
                break
        # 4
        for x1 in range(x-1,-1,-1):
            if state[y][x1] == 'L':
                break
            if state[y][x1] == '#':
                neighbours += 1
                break
        # 5
        for x1 in range(x+1,x_):
            if state[y][x1] == 'L':
                break
            if state[y][x1] == '#':
                neighbours += 1
                break
        # 1
        for y1,x1 in zip(range(y-1,-1,-1), range(x-1,-1,-1)):
            if state[y1][x1] == 'L':
                break
            if state[y1][x1] == '#':
                neighbours += 1
                break
        # 3
        for y1,x1 in zip(range(y-1,-1,-1), range(x+1,x_)):
            if state[y1][x1] == 'L':
                break
            if state[y1][x1] == '#':
                neighbours += 1
                break
        # 6
        for y1,x1 in zip(range(y+1,y_), range(x-1,-1,-1)):
            if state[y1][x1] == 'L':
                break
            if state[y1][x1] == '#':
                neighbours += 1
                break
        # 8
        for y1,x1 in zip(range(y+1,y_), range(x+1,x_)):
            if state[y1][x1] == 'L':
                break
            if state[y1][x1] == '#':
                neighbours += 1
                break
        return neighbours

    def generation_2(state, size):
        x_, y_ = size
        state2 = [['?' for i in range(x_)] for i in range(y_)]
        changed = False

        for x,y in itertools.product(range(x_), range(y_)):
            state2[y][x] = state[y][x]
            if state[y][x] == 'L' and seen_neighbours(state, size, x, y) == 0:
                state2[y][x] = '#'
                changed = True
            if state[y][x] == '#' and seen_neighbours(state, size, x, y) >= 5:
                state2[y][x] = 'L'
                changed = True
        return state2, changed

    print("\nDay 11, Part 1")
    state, size = init(data)
    changed = True
    gen_count = 0
    while changed:
        gen_count += 1
        (state,changed) = generation_2(state, size)
        # print(f"\n\ngeneration {gen_count}:")
        # for line in state:
        #     print("".join(line))
    chars = "".join(["".join(line) for line in state])
    print(f"finally occupied chairs: {chars.count('#')}")


def day12():
    data = '''\
F10
N3
F7
R90
F11
'''
    data = requests.get('https://adventofcode.com/2020/day/12/input', headers=advent_headers).text
    directions = ["east", "south", "west", "north"]
    # part 1
    state = {"direction": "east", "pos_e": 0, "pos_n": 0}
    print(f"init state: {state}")
    for command in data.splitlines():
        op = command[0]
        arg = int(command[1:])

        if op == 'N':
            state["pos_n"] += arg
        elif op == 'S':
            state["pos_n"] -= arg
        elif op == 'E':
            state["pos_e"] += arg
        elif op == 'W':
            state["pos_e"] -= arg
        elif op == 'L':
            if arg % 90:
                print(f"{op}{arg} unsupported")
            state["direction"] = directions[ (directions.index(state["direction"]) - int((arg / 90))) % len(directions) ]
        elif op == 'R':
            if arg % 90:
                print(f"{op}{arg} unsupported")
            state["direction"] = directions[ (directions.index(state["direction"]) + int((arg / 90))) % len(directions) ]
        elif op == 'F':
            if state["direction"] == "east":  state["pos_e"] += arg
            if state["direction"] == "west":  state["pos_e"] -= arg
            if state["direction"] == "north": state["pos_n"] += arg
            if state["direction"] == "south": state["pos_n"] -= arg
        print(f"{command} --> {state}")
    f_dist = abs(state["pos_e"]) + abs(state["pos_n"])
    print(f"final distance: {f_dist}")


    # part 2
    state = {"direction": "east", "pos_e": 0, "pos_n": 0, "wp_e": 10, "wp_n": 1}
    print(f"init state: {state}")
    for command in data.splitlines():
        op = command[0]
        arg = int(command[1:])

        if op == 'N':
            state["wp_n"] += arg
        elif op == 'S':
            state["wp_n"] -= arg
        elif op == 'E':
            state["wp_e"] += arg
        elif op == 'W':
            state["wp_e"] -= arg
        elif op == 'R':
            if arg % 90:
                print(f"{op}{arg} unsupported")
            turns = arg // 90
            for i in range(turns):
                wp_n, wp_e = state["wp_n"], state["wp_e"]
                state["wp_n"] = -wp_e
                state["wp_e"] = wp_n
        elif op == 'L':
            if arg % 90:
                print(f"{op}{arg} unsupported")
            turns = arg // 90
            for i in range(turns):
                wp_n, wp_e = state["wp_n"], state["wp_e"]
                state["wp_n"] = wp_e
                state["wp_e"] = -wp_n
        elif op == 'F':
            state["pos_e"] += arg * state["wp_e"]
            state["pos_n"] += arg * state["wp_n"]
        else:
            print(f"unsupported command {command}")

        print(f"{command} --> {state}")
    f_dist = abs(state["pos_e"]) + abs(state["pos_n"])
    print(f"final distance: {f_dist}")


def day13():
    data: str = '''\
        939
        7,13,x,x,59,x,31,19
        '''.strip()
    data: str = '''\
        939
        67,7,x,59,61
        '''.strip()

    data: str = requests.get('https://adventofcode.com/2020/day/13/input', headers=advent_headers).text
    inputlines: List[str] = data.splitlines()

    # Part 1
    cur_time: int = int(inputlines[0].strip())
    buslines: List[int] = [int(i) for i in inputlines[1].split(',') if i != 'x']
    print(f"init: time {cur_time}, busses {buslines}")

    next_bus: Tuple[int,int] = 1000,1000
    for bus in buslines:
        last_dep: int = cur_time % bus
        next_dep: int = bus - last_dep
        print(f"bus {bus}: last {last_dep}, next {next_dep} min")
        if next_dep < next_bus[0]:
            next_bus = (next_dep, bus)
    print(f"next bus {next_bus[1]} in {next_bus[0]} min => {next_bus[0] * next_bus[1]}")

    # Part 2
    def extended_euclid(a: Tuple[int], m: Tuple[int]) -> int:
        from operator import mul
        from functools import reduce

        def xgcd(a: int, b: int) -> Tuple[int, int, int]:
            """return (g, x, y) such that a*x + b*y = g = gcd(a, b)"""
            x0, x1, y0, y1 = 0, 1, 1, 0
            while a != 0:
                (q, a), b = divmod(b, a), a
                y0, y1 = y1, y0 - q * y1
                x0, x1 = x1, x0 - q * x1
            return b, x0, y0

        assert len(a) == len(m)
        length: int = len(a)
        MM: int = reduce(mul, m, 1)
        M: Tuple[int, ...] = tuple([MM//m[i] for i in range(length)])

        s: List[int] = [0] * length
        e: List[int] = [0] * length
        for i in range(length):
            g_, r_, s[i] = xgcd(m[i], M[i])
            if g_ != 1: print(f"ERROR, g = {g_} != 1")
            e[i] = s[i] * M[i]
            print(f"a_i = {a[i]}, m_i = {m[i]}, M_i = {M[i]}, r_i = {r_}, s_i = {s[i]} => e_i = {e[i]}")
        x: int = sum([a[i] * e[i] for i in range(length)])
        print(f"yields x = {x} = {x % MM} mod {MM}")
        print(f"yields -x = {-x} = {-x % MM} mod {MM}")
        return -x % MM

    constraints_in: List = [(int(i) if i!= 'x' else 0) for i in inputlines[1].split(',')]
    constraints: List[Tuple[int, int]] = [i for i in filter(lambda x: x[1], enumerate(constraints_in))]
    a: Tuple[int] = tuple([i[0] for i in constraints])
    m: Tuple[int] = tuple([i[1] for i in constraints])
    print(f"\n\ninit: {constraints_in},\nshortened: {constraints}")
    x: int = extended_euclid(a, m)

    def check(ts: int, constraints: List[Tuple[int, int]]) -> bool:
        for item in constraints:
            if 0 != (ts + item[0]) % item[1]:
                return False
        return True
    if check(x, constraints):
        print(f"solution: {x}")


def day14():
    data = '''\
        mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        mem[8] = 11
        mem[7] = 101
        mem[8] = 0
        '''.strip()
    data = requests.get('https://adventofcode.com/2020/day/14/input', headers=advent_headers).text

    mem: Dict[int, int] = {}
    # part 1
    for line in data.splitlines():
        lval, rval = line.strip().split(" = ")
        if lval == "mask":
            mask_one: int = int(rval.replace("X", "0"), 2)
            mask_zero: int = int(rval.replace("X", "1"), 2)
            print(f"mask = {rval}")
        else:
            memloc: int = lval.replace("mem[", "").replace("]", "")
            value: int = int(rval)
            newval: int = (value | mask_one) & mask_zero
            mem[memloc] = newval
            print(f"mem[{memloc}] = {value} ~> {newval}")
    print(f"final memory: {mem}")
    print(f"final result: {sum(mem.values())}")

    # part 2
    def gen_floating_bitmasks(inputmask: str) -> List[str]:
        result: List[str] = list()
        i = inputmask.find('X')
        if i == -1:
            return [inputmask]
        else:
            mask = list(inputmask)
            mask[i] = '0'
            result += gen_floating_bitmasks("".join(mask))
            mask[i] = '1'
            result += gen_floating_bitmasks("".join(mask))
        return result

    mem: Dict[int, int] = {}
    masklist = []
    for line in data.splitlines():
        lval, rval = line.strip().split(" = ")
        if lval == "mask":
            mask_one: int = int(rval.replace("X", "0"), 2)
            mask_float: str = rval.replace("1", ".").replace("0", ".") # only X and 0S
            masklist: List[str] = gen_floating_bitmasks(mask_float)
            print(f"mask {rval} => mask_one {mask_one}, masklist {masklist}")
        else:
            value: int = int(rval)

            memloc_: int = int(lval.replace("mem[", "").replace("]", ""))
            memloc: int = memloc_ | mask_one
            # mem locations with floating bits:
            locs: List[int] = []
            for mask in masklist:
                mask_1 = int(mask.replace('.', '0'), 2)
                mask_0 = int(mask.replace('.', '1'), 2)
                locs.append((memloc | mask_1) & mask_0)
            for loc in locs:
                mem[loc] = value
            print(f"mem[{memloc_} ~> {locs}] = {value}")
    print(f"final memory: {mem}")
    print(f"final result: {sum(mem.values())}")


def day15():

    def play_game(data: str, round_max: int, debug: bool=False) -> int:
        start_values: List[int] = [int(i) for i in data.strip().split(',')]
        memory: Dict[int, Tuple[int, int]] = {}
        last_num: int = 0

        for round,num in enumerate(start_values):
            memory[num] = (round+1,0)
            if debug: print(f"round {round+1:4d}: {num}")
            last_num = num
        for round in range(len(start_values)+1, round_max+1):
            last, before = memory.get(last_num, (round,0))
            if debug: print(f"round {round:4d}: {last_num} -> memory[{last_num}] = ({last}, {before})", end='')
            if before == 0: # called the first time
                num = 0
            else:
                num = last - before

            num_last, num_before = memory.get(num, (0,0))
            memory[num] = (round, num_last)
            if debug: print(f" -> {num} ... write memory[{num}] = ({round}, {num_last})")
            last_num = num
        return last_num

    data = [
        '0,3,6',
        '1,3,2',
        '2,1,3',
        '1,2,3',
        '2,3,1',
        '3,2,1',
        '3,1,2',
        '6,19,0,5,7,13,1'
    ]
    max_rounds = 2020
    for dat in data:
        num = play_game(dat, max_rounds)
        print(f"play_game({dat}, {max_rounds}) --> {num}")
    max_rounds = 30000000
    for dat in data:
        num = play_game(dat, max_rounds)
        print(f"play_game({dat}, {max_rounds}) --> {num}")


def day16():
    data = '''\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
'''.strip()

    data = '''\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
'''.strip()

    data = requests.get('https://adventofcode.com/2020/day/16/input', headers=advent_headers).text
    block_notes, block_my, block_nearby = data.split("\n\n")

    notes = {}
    for line in block_notes.splitlines():
        name, ranges = line.strip().split(":")
        notes[name] = []
        rangelist = ranges.strip().split(" or ")
        for item in rangelist:
            rangemin, rangemax = item.split("-")
            notes[name].append((int(rangemin), int(rangemax)))

    assert len(block_my.splitlines()) == 2
    assert block_my.splitlines()[0] == "your ticket:"
    my_ticket = tuple([int(i) for i in block_my.splitlines()[1].strip().split(",")])

    assert block_nearby.splitlines()[0] == "nearby tickets:"
    nearby: List[Tuple[int, ...]] = []
    valid: List[Tuple[int, ...]] = []
    for line in block_nearby.splitlines()[1:]:
        nearby.append(tuple([int(i) for i in line.split(",")]))

    print(f"notes: {notes}\nmine: {my_ticket}\nnearby: {nearby}")

    scanning_error_rate = 0
    for ticket in nearby:
        valid_ticket = True
        for i in ticket:
            valid_num = False
            for note in sum(notes.values(), []):
                if note[0] <= i <= note[1]:
                    valid_num = True
                    break
            if not valid_num:
                print(f"found invalid number {i}")
                scanning_error_rate += i
                valid_ticket = False
        if valid_ticket:
            valid.append(ticket)

    print(f"scanning_error_rate {scanning_error_rate}\n\n")
    print(f"valid tickets {valid}")

    def check_category(num: int, checks: List[Tuple[int, int]]) -> bool:
        # print(f"check_category({num}, {checks}) ", end="")
        for note in checks:
            if note[0] <= num <= note[1]:
                # print(f" --> True")
                return True
        # print(f" --> False")
        return False

    categories = len(notes.keys())
    assert len(valid[0]) == len(notes.keys()) == categories
    matching = []

    for i in range(categories):
        cat_values = []
        for ticket in valid:
            cat_values.append(ticket[i])
        cat_matching = [
            check_category(num, notes[cat]) for num in cat_values for cat in notes.keys()
        ]
        # print(f"cat_values {cat_values}, cat_matching {cat_matching}")
        cat_possible = [cat for cat in notes.keys() if all([
            check_category(num, notes[cat]) for num in cat_values
        ])]
        matching.append({
            "column": i,
            "cats": cat_possible,
            "values": cat_values
        })
    print(f"matching {matching}")

    change: bool = True
    found_matches: Dict[str, int] = {}
    while change:
        change = False
        for match in filter(lambda m: 1 == len(m['cats']), matching):
            change = True
            cat: str = match['cats'][0]
            print(f"handle match {cat}")
            found_matches[cat] = match['column']
            matching.remove(match)
            for entry in matching:
                entry['cats'].remove(cat)

    assert notes.keys() == found_matches.keys()
    print(f"found_matches {found_matches}")

    my_ticket_cats = {}
    result_product = 1
    for cat, column in found_matches.items():
        my_ticket_cats[cat] = my_ticket[column]
        if cat.startswith("departure"):
            result_product *= my_ticket[column]
    print(f"my ticket: {my_ticket_cats}\nresult is {result_product}")


def day17_1():
    data = '''\
.#.
..#
###
'''
    data = requests.get('https://adventofcode.com/2020/day/17/input', headers=advent_headers).text

    cycle_limit: int = 6
    lines: List[str] = [i.strip() for i in data.strip().splitlines() if i]

    # state is not an array, but simply a list of points
    state: List[Tuple[int, int, int]] = []

    # init, z=0
    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            point = (x, y, 0)
            if char == "#":
                state.append(point)

    def print_state(state: List[Tuple[int, int, int]]) -> str:
        outstr = ""
        x_range: List[int] = sorted(list(set([x for (x, y, z) in state])))
        y_range: List[int] = sorted(list(set([y for (x, y, z) in state])))
        z_range: List[int] = sorted(list(set([z for (x, y, z) in state])))
        for z in z_range:
            outstr += f"\nz={z}\n"
            for y in y_range:
                x_line = ['#' if (x, y, z) in state else '.' for x in x_range]
                outstr += "".join(x_line) + "\n"
        return outstr

    def neighbours(state: List[Tuple[int, int, int]], x: int, y: int, z: int) -> int:
        neighbour_list: List[Tuple[int, int, int]] = []
        for x_ in [x-1, x, x+1]:
            for y_ in [y-1, y, y+1]:
                for z_ in [z-1, z, z+1]:
                    point = (x_, y_, z_)
                    neighbour_list.append(point)
        neighbour_list.remove((x,y,z))
        return len(list(filter(lambda point: point in state, neighbour_list)))

    def is_active(state: List[Tuple[int, int, int]], x: int, y: int, z: int) -> bool:
        return (x, y, z) in state

    def become_active(state: List[Tuple[int, int, int]], x: int, y: int, z: int) -> bool:
        return (is_active(state, x, y, z) and neighbours(state, x, y, z) in [2,3]) \
               or (not is_active(state, x, y, z) and neighbours(state, x, y, z) == 3)

    def dimensions(state: List[Tuple[int, int, int]]) -> Dict[str, int]:
        result: Dict[str, int] = {
            "xmin": 0, "xmax": 0,
            "ymin": 0, "ymax": 0,
            "zmin": 0, "zmax": 0,
        }
        for x, y, z in state:
            if x < result["xmin"]: result["xmin"] = x
            if x > result["xmax"]: result["xmax"] = x

            if y < result["ymin"]: result["ymin"] = y
            if y > result["ymax"]: result["ymax"] = y

            if z < result["zmin"]: result["zmin"] = z
            if z > result["zmax"]: result["zmax"] = z
        return result

    def cycle(old: List[Tuple[int, int, int]]) -> List[Tuple[int, int, int]]:
        new: List[Tuple[int, int, int]] = []
        dim = dimensions(old)
        for x in range(dim["xmin"]-1, dim["xmax"]+2):
            for y in range(dim["ymin"]-1, dim["ymax"]+2):
                for z in range(dim["zmin"]-1, dim["zmax"]+2):
                    if become_active(old, x, y, z):
                        new.append((x, y, z))
        return new

    print(f"init:")
    print(print_state(state))

    for i in range(1,cycle_limit+1):
        print(f"After cycle {i}:")
        state = cycle(state)
        print(f"active cubes: {len(state)}")
        #print(print_state(state))


def day17_2():
    data = '''\
.#.
..#
###
'''
    data = requests.get('https://adventofcode.com/2020/day/17/input', headers=advent_headers).text

    cycle_limit: int = 6
    lines: List[str] = [i.strip() for i in data.strip().splitlines() if i]
    # state is not an array, but simply a list of points
    state: List[Tuple[int, int, int, int]] = []

    # init, z=0, w=0
    for y,line in enumerate(lines):
        for x,char in enumerate(line):
            point = (x, y, 0, 0)
            if char == "#":
                state.append(point)

    def print_state(state: List[Tuple[int, int, int, int]]) -> str:
        outstr = ""
        x_range: List[int] = sorted(list(set([x for (x, y, z, w) in state])))
        y_range: List[int] = sorted(list(set([y for (x, y, z, w) in state])))
        z_range: List[int] = sorted(list(set([z for (x, y, z, w) in state])))
        w_range: List[int] = sorted(list(set([w for (x, y, z, w) in state])))
        for w in w_range:
            for z in z_range:
                outstr += f"\nz={z}, w={w}\n"
                for y in y_range:
                    x_line = ['#' if (x, y, z, w) in state else '.' for x in x_range]
                    outstr += "".join(x_line) + "\n"
        return outstr

    def neighbours(state: List[Tuple[int, int, int, int]], x: int, y: int, z: int, w: int) -> int:
        neighbour_list: List[Tuple[int, int, int, int]] = []
        for x_ in [x-1, x, x+1]:
            for y_ in [y-1, y, y+1]:
                for z_ in [z-1, z, z+1]:
                    for w_ in [w-1, w, w+1]:
                        point = (x_, y_, z_, w_)
                        neighbour_list.append(point)
        neighbour_list.remove((x, y, z, w))
        return len(list(filter(lambda point: point in state, neighbour_list)))

    def is_active(state: List[Tuple[int, int, int, int]], x: int, y: int, z: int, w: int) -> bool:
        return (x, y, z, w) in state

    def become_active(state: List[Tuple[int, int, int, int]], x: int, y: int, z: int, w: int) -> bool:
        return (is_active(state, x, y, z, w) and neighbours(state, x, y, z, w) in [2,3]) \
               or (not is_active(state, x, y, z, w) and neighbours(state, x, y, z, w) == 3)

    def dimensions(state: List[Tuple[int, int, int, int]]) -> Dict[str, int]:
        result: Dict[str, int] = {
            "xmin": 0, "xmax": 0,
            "ymin": 0, "ymax": 0,
            "zmin": 0, "zmax": 0,
            "wmin": 0, "wmax": 0,
        }
        for x, y, z, w in state:
            if x < result["xmin"]: result["xmin"] = x
            if x > result["xmax"]: result["xmax"] = x

            if y < result["ymin"]: result["ymin"] = y
            if y > result["ymax"]: result["ymax"] = y

            if z < result["zmin"]: result["zmin"] = z
            if z > result["zmax"]: result["zmax"] = z

            if w < result["wmin"]: result["wmin"] = w
            if w > result["wmax"]: result["wmax"] = w
        return result

    def cycle(old: List[Tuple[int, int, int, int]]) -> List[Tuple[int, int, int, int]]:
        new: List[Tuple[int, int, int, int]] = []
        dim = dimensions(old)
        for x in range(dim["xmin"]-1, dim["xmax"]+2):
            for y in range(dim["ymin"]-1, dim["ymax"]+2):
                for z in range(dim["zmin"]-1, dim["zmax"]+2):
                    for w in range(dim["wmin"]-1, dim["wmax"]+2):
                        if become_active(old, x, y, z, w):
                            new.append((x, y, z, w))
        return new

    print(f"init:")
    print(print_state(state))

    for i in range(1, cycle_limit+1):
        print(f"After cycle {i}:")
        state = cycle(state)
        print(f"active cubes: {len(state)}")
        # print(print_state(state))


def day18():
    data = '''\
1 + 2 * 3 + 4 * 5 + 6
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2
'''
    # data = requests.get('https://adventofcode.com/2020/day/18/input', headers=advent_headers).text

    # Part 1
    def scan_parenthesis(term: str) -> int:
        assert term[0] == '('
        level = 0
        i = 1
        while i < len(term):
            char = term[i]
            if char == '(':
                level += 1
            elif char == ')':
                if level > 0:
                    level -= 1
                else:
                    return i
            i += 1
        raise ValueError(f"non-matching parenthesis in term '{term}'")

    def op(op_char: str, left: int, right: int) -> int:
        result: int
        if op_char == '+':
            result = left + right
        elif op_char == '*':
            result = left * right
        else:
            raise ValueError(f"unexpected operator '{op_char}'")
        # print(f"op({op_char}, {left}, {right}) -> {result}")
        return result

    def eval_term(term: str) -> int:
        if term.isdigit():
            return int(term)
        if term[0].isdigit() and term[1].isspace() and term[3].isspace():
            return op(term[2], int(term[0]), eval_term(term[4:]))
        if term[0] == '(':
            end_index = scan_parenthesis(term)
            assert term[end_index] == ')'
            if end_index+1 == len(term):
                return eval_term(term[1:end_index])
            assert term[end_index+1].isspace()
            assert term[end_index+2] in ['+', '*']
            assert term[end_index+3].isspace()
            return op(term[end_index+2], eval_term(term[1:end_index]), eval_term(term[end_index+4:]))
        raise ValueError(f"unexpected case")

    # result_sum = 0
    # for line in data.strip().splitlines():
    #     # very stupid hack, because my evaluation is right-associative and not left-associative
    #     reversed_term = line.strip()[::-1].replace('(', 'O').replace(')', '(').replace('O', ')')
    #     result = eval_term(reversed_term)
    #     result_sum += result
    #     print(f"{result} = {line}")
    # print(f"result sum {result_sum}")

    # Part 1a


    class Stack:
        def __init__(self):
            self.items = []

        def isEmpty(self):
            return self.items == []

        def push(self, item):
            self.items.append(item)

        def pop(self):
            return self.items.pop()

        def peek(self):
            return self.items[len(self.items)-1]

        def size(self):
            return len(self.items)

    class BinaryTree:
        def __init__(self,rootObj):
            self.key = rootObj
            self.leftChild = None
            self.rightChild = None

        def insertLeft(self,newNode):
            if self.leftChild == None:
                self.leftChild = BinaryTree(newNode)
            else:
                t = BinaryTree(newNode)
                t.leftChild = self.leftChild
                self.leftChild = t

        def insertRight(self,newNode):
            if self.rightChild == None:
                self.rightChild = BinaryTree(newNode)
            else:
                t = BinaryTree(newNode)
                t.rightChild = self.rightChild
                self.rightChild = t

        def getRightChild(self):
            return self.rightChild

        def getLeftChild(self):
            return self.leftChild

        def setRootVal(self,obj):
            self.key = obj

        def getRootVal(self):
            return self.key

        def __str__(self):
            return f"[ {self.getLeftChild()} {self.getRootVal()} {self.getRightChild()}]"

    def buildParseTree(fpexp):
        fpexp = fpexp.replace("(", "( ").replace(")", " )")
        fplist = fpexp.split()
        pStack = Stack()
        eTree = BinaryTree('')
        pStack.push(eTree)
        currentTree = eTree

        for i in fplist:
            if i == '(':
                currentTree.insertLeft('')
                pStack.push(currentTree)
                currentTree = currentTree.getLeftChild()

            elif i in ['+', '-', '*', '/']:
                currentTree.setRootVal(i)
                currentTree.insertRight('')
                pStack.push(currentTree)
                currentTree = currentTree.getRightChild()

            elif i == ')':
                currentTree = pStack.pop()

            elif i not in ['+', '-', '*', '/', ')']:
                try:
                    currentTree.setRootVal(int(i))
                    parent = pStack.pop()
                    currentTree = parent

                except ValueError:
                    raise ValueError("token '{}' is not a valid integer".format(i))

        return eTree

    def parenthesis_sublist(term: List[str]) -> List:
        level = 0
        i = 1
        while i < len(term):
            char = term[i]
            if char == '(':
                level += 1
            elif char == ')':
                if level > 0:
                    level -= 1
                else:
                    return list()
            i += 1
        raise ValueError(f"non-matching parenthesis in term '{term}'")

    def postorder(tree):
        if tree != None:
            postorder(tree.getLeftChild())
            postorder(tree.getRightChild())
            print(tree.getRootVal())

    result_sum = 0
    for line in data.strip().splitlines():
        pt = buildParseTree(line)
        postorder(pt)  #defined and explained in the next section
        print(pt)
        return


def day19():
    data = '''\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
'''
    data = requests.get('https://adventofcode.com/2020/day/19/input', headers=advent_headers).text

    rules_input, data_input = data.strip().split("\n\n")
    rules_input = rules_input.splitlines()
    data_input = data_input.splitlines()

    class Rule(object):
        id: str
        type: str

        content_leaf: str
        content_disjunction: List
        content_conjunction: List[int]

        def __init__(self, line: str):
            if ":" not in line:
                print(f"no ':' in line '{line}'")
                raise ValueError()
            rule_number, rule_content = line.strip().split(":")
            self.id = rule_number

            if '"' in rule_content:
                self.type = "l"
                self.content_leaf = rule_content.replace('"', '').strip()
            elif '|' in rule_content:
                self.type = "o"
                self.content_disjunction = [Rule(f"{self.id}_{i}: {s}") for i,s in enumerate(rule_content.split('|'))]
            else:
                self.type = "r"
                self.content_conjunction = [int(i) for i in rule_content.split()]

        def __repr__(self):
            if self.type == "l":
                content = self.content_leaf
            elif self.type == "o":
                content = self.content_disjunction
            elif self.type == "r":
                content = self.content_conjunction
            return f"Rule({self.id}, {self.type}, {content})"

        def check(self, input: str, all_rules: Dict = {}) -> bool:
            if self.type == "l":
                return self.content_leaf == input
            elif self.type == "o":
                return any([r.check(input, all_rules) for r in self.content_disjunction])
            elif self.type == "r":
                subrules = [all_rules[i] for i in self.content_conjunction]
                if len(subrules) == 0:
                    raise ValueError("invalid: 0 subrules")
                elif len(subrules) == 1:
                    return subrules[0].check(input, all_rules)
                elif len(subrules) == 2:
                    if subrules[0].type == 'l':
                        pivot1 = 1
                        return subrules[0].check(input[:pivot1], all_rules) and \
                               subrules[1].check(input[pivot1:], all_rules)
                    elif subrules[1].type == 'l':
                        pivot1 = len(input)-1
                        return subrules[0].check(input[:pivot1], all_rules) and \
                               subrules[1].check(input[pivot1:], all_rules)
                    else:
                        for pivot in range(0,len(input)):
                            if subrules[0].check(input[:pivot], all_rules) and \
                               subrules[1].check(input[pivot:], all_rules):
                                return True
                elif len(subrules) == 3:
                    if subrules[0].type == 'l' and subrules[2].type == 'l':
                        pivot1 = 1
                        pivot2 = len(input)-1
                        return subrules[0].check(input[:pivot1], all_rules) and \
                               subrules[1].check(input[pivot1:pivot2], all_rules) and \
                               subrules[2].check(input[pivot2:], all_rules)
                    elif subrules[0].type == 'l':
                        pivot1 = 1
                        for pivot2 in range(pivot1, len(input)):
                            if subrules[0].check(input[:pivot1], all_rules) and \
                               subrules[1].check(input[pivot1:pivot2], all_rules) and \
                               subrules[2].check(input[pivot2:], all_rules):
                                return True
                    elif subrules[2].type == 'l':
                        pivot2 = len(input)-1
                        for pivot1 in range(0, pivot2):
                            if subrules[0].check(input[:pivot1], all_rules) and \
                               subrules[1].check(input[pivot1:pivot2], all_rules) and \
                               subrules[2].check(input[pivot2:], all_rules):
                                return True
                    else:
                        for pivot1 in range(0, len(input)):
                            for pivot2 in range(pivot1, len(input)):
                                if subrules[0].check(input[:pivot1], all_rules) and \
                                   subrules[1].check(input[pivot1:pivot2], all_rules) and \
                                   subrules[2].check(input[pivot2:], all_rules):
                                    return True
                else:
                    raise NotImplementedError("missing: >2 subrules")
                return False
            else:
                raise ValueError("invalid Rule object")

    rules: Dict[int, Rule] = {}
    for rule_line in rules_input:
        r: Rule = Rule(rule_line)
        rules[int(r.id)] = r

    print(rules)

    count_matches = 0
    for input in data_input:
        if rules[0].check(input, rules):
            count_matches += 1
    print(f"matching inputs: {count_matches}")

    # Part 2
    rules[8] = Rule('8: 42 | 42 8')
    rules[11] = Rule('11: 42 31 | 42 11 31')
    count_matches = 0
    for input in data_input:
        if rules[0].check(input, rules):
            print(f"+ {input}")
            count_matches += 1
        else:
            print(f"- {input}")
    print(f"matching inputs: {count_matches}")


day19()
