import re
from .helper import get_input_data

def solve():
    example = """xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"""
    input_text = get_input_data(2024, 3, example)

    pattern = re.compile(r'mul\((?P<first>\d{1,3}),(?P<second>\d{1,3})\)')

    # part 1
    multiplications = pattern.findall(input_text)
    print(sum([int(a)*int(b) for a,b in multiplications]))

    # part 2
    example = """xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"""
    input_text = get_input_data(2024, 3, example)

    pattern = re.compile(r"(?P<start>do\(\))|(?P<stop>don't\(\))|(?P<mul>mul\((?P<first>\d{1,3}),(?P<second>\d{1,3})\))")
    state_do = True
    acc = 0
    for m in pattern.finditer(input_text):
        if m.lastgroup == 'start':
            state_do = True
        elif m.lastgroup == 'stop':
            state_do = False
        elif m.lastgroup == 'mul' and state_do:
           acc += int(m.group('first')) * int(m.group('second'))
    print(acc)
