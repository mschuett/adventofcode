from .helper import get_input_data
import textwrap
import fractions
from typing import Union, Tuple
from days.day08 import Coordinate


class ClawMachine(object):
    a: Coordinate
    b: Coordinate
    prize: Coordinate

    def __init__(self, input_text: str):
        lines = input_text.strip().splitlines()
        assert len(lines) == 3
        assert lines[0].startswith('Button A: X+')
        assert lines[1].startswith('Button B: X+')
        assert lines[2].startswith('Prize: X=')

        words = lines[0].split(' ')
        self.a = Coordinate(
            int(words[2].split('+')[1].strip(',')),
            int(words[3].split('+')[1]))
        words = lines[1].split(' ')
        self.b = Coordinate(
            int(words[2].split('+')[1].strip(',')),
            int(words[3].split('+')[1]))
        words = lines[2].split(' ')
        self.prize = Coordinate(
            int(words[1].split('=')[1].strip(',')),
            int(words[2].split('=')[1]))

    def construct_path(self) -> Union[Tuple[int,int],None]:
        # the iterative "part one" solution, trying to avoid the 'real' linear algebra,
        # prefer (cheaper) button b, so press A only until we get the right ratio to continue with B
        ratio_b = fractions.Fraction(self.b.x, self.b.y)
        button_count_a = button_count_b = 0
        button_press_limit = 100

        cursor = Coordinate(0, 0)
        missing_path = cursor - self.prize
        prize_ratio = fractions.Fraction(missing_path.x, missing_path.y)

        while cursor < self.prize and prize_ratio != ratio_b and button_count_a < button_press_limit:
            button_count_a += 1
            cursor += self.a
            missing_path = cursor - self.prize
            if missing_path.y == 0: break  # hacky
            prize_ratio = fractions.Fraction(missing_path.x, missing_path.y)
        while cursor < self.prize and button_count_b < button_press_limit:
            button_count_b += 1
            cursor += self.b

        if cursor == self.prize:
            # double check construction
            assert (self.a * button_count_a) + (self.b * button_count_b) == self.prize
            return button_count_a, button_count_b
        else:
            return None

    def get_price(self) -> Union[int,None]:
        p = self.construct_path()
        if not p:
            print('prize unreachable')
            return 0
        a, b = p
        tokens = b + a * 3
        print(f"{a} times A and {b} times B, costing {tokens} tokens")
        return tokens

    def __repr__(self):
        return f"{self.__class__.__name__}(A {self.a}, B {self.b}, Prize {self.prize})"


class SuperClawMachine(ClawMachine):
    def __init__(self, input_text: str):
        super().__init__(input_text)
        self.prize = Coordinate(self.prize.x+10000000000000, self.prize.y+10000000000000)
        self.button_press_limit = 0

    def construct_path(self) -> Union[Tuple[int,int],None]:
        # the 'real' analytical solution, solving the two linear equations
        button_count_b = fractions.Fraction(self.a.y * self.prize.x - self.a.x * self.prize.y, self.a.y * self.b.x - self.a.x * self.b.y)
        button_count_a = fractions.Fraction(self.prize.x - self.b.x * button_count_b, self.a.x)
        # print(f"A={button_count_a}, B={button_count_b}")

        if button_count_a.is_integer() and button_count_b.is_integer():
            # double check construction
            assert (self.a * int(button_count_a)) + (self.b * int(button_count_b)) == self.prize
            return int(button_count_a), int(button_count_b)
        else:
            return None

def solve():
    example = """\
    Button A: X+94, Y+34
    Button B: X+22, Y+67
    Prize: X=8400, Y=5400
    
    Button A: X+26, Y+66
    Button B: X+67, Y+21
    Prize: X=12748, Y=12176
    
    Button A: X+17, Y+86
    Button B: X+84, Y+37
    Prize: X=7870, Y=6450
    
    Button A: X+69, Y+23
    Button B: X+27, Y+71
    Prize: X=18641, Y=10279
    """
    input_text = get_input_data(2024, 13, example)

    # part 1
    sum_tokens = 0
    for snippet in textwrap.dedent(input_text).split("\n\n"):
        a = ClawMachine(snippet)
        # print(a)
        sum_tokens += a.get_price()
    print(f"total tokens: {sum_tokens}")

    # part 2
    sum_tokens = 0
    for snippet in textwrap.dedent(input_text).split("\n\n"):
        a = SuperClawMachine(snippet)
        # print(a)
        sum_tokens += a.get_price()
    print(f"total tokens: {sum_tokens}")
