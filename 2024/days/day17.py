import collections

from .helper import get_input_data
from typing import *
import textwrap

class Computer(object):
    ra: int
    rb: int
    rc: int
    code: list[int]
    pc: int
    output: list[int]

    def __init__(self, input_text: str):
        regs, prog = textwrap.dedent(input_text).split('\n\n')
        self.ra, self.rb, self.rc = [int(line.split(' ')[-1]) for line in regs.splitlines()]
        self.code = [int(i) for i in prog.strip().strip("[").strip("]").split(' ')[1].split(',')]
        self.pc = 0
        self.output = []

    def __repr__(self):
        code = ",".join([str(i) for i in self.code])
        output = ",".join([str(i) for i in self.output])
        return f'Computer({self.ra}, {self.rb}, {self.rc}, code (len {self.pc}) {code}): {output}'

    def run(self):
        while self.step():
            pass

    def step(self) -> bool:
        # read instruction
        if self.pc >= len(self.code):
            return False  # halt
        instruction = self.code[self.pc]
        operand = self.code[self.pc + 1]
        combo = {
            0: 0,
            1: 1,
            2: 2,
            3: 3,
            4: self.ra,
            5: self.rb,
            6: self.rc,
        }

        match instruction:
            case 0:  # adv
                self.ra = self.ra >> combo[operand]
            case 1:  # bxl
                self.rb = self.rb ^ operand
            case 2:  # bst
                self.rb = combo[operand] % 8
            case 3:  # jnz
                if self.ra != 0:
                    self.pc = operand - 2
            case 4:  # bxc
                self.rb = self.rb ^ self.rc
            case 5:  # out
                self.output.append(combo[operand] % 8)
            case 6:  # bdv
                self.rb = self.ra >> combo[operand]
            case 7:  # cdv
                self.rc = self.ra >> combo[operand]
            case _:
                raise ValueError(f'Invalid instruction: {instruction}')

        self.pc += 2
        return True

    def prettyprint_program(self):
        i = 0
        combo = {
            0: "0",
            1: "1",
            2: "2",
            3: "3",
            4: "ra",
            5: "rb",
            6: "rc",
        }

        while i < len(self.code):
            instruction, operand = self.code[i:i+2]
            match instruction:
                case 0:
                    print(f'{i:2} adv, ra = ra >> {combo[operand]}')
                case 1:
                    print(f'{i:2} bxl, rb = rb ^ {operand}')
                case 2:
                    print(f'{i:2} bst, rb = {combo[operand]} % 8')
                case 3:
                    print(f'{i:2} jnz, if ra != 0 jmp {operand}')
                case 4:
                    print(f'{i:2} bxc, rb = rb ^ rc')
                case 5:
                    print(f'{i:2} out, print {combo[operand]} % 8')
                case 6:
                    print(f'{i:2} bdv, rb = ra >> {combo[operand]}')
                case 7:
                    print(f'{i:2} cdv, rc = ra >> {combo[operand]}')
            i += 2


def test_cases():
    # test_cases
    tc = Computer("""\
            Register A: 0
            Register B: 0
            Register C: 9
            
            Program: 2,6""")
    tc.run()
    assert tc.rb == 1

    tc = Computer("""\
            Register A: 10
            Register B: 0
            Register C: 0
            
            Program: 5,0,5,1,5,4""")
    tc.run()
    assert tc.output == [0,1,2]

    tc = Computer("""\
            Register A: 2024
            Register B: 0
            Register C: 0
            
            Program: 0,1,5,4,3,0""")
    tc.run()
    assert tc.output == [4,2,5,6,7,7,7,7,3,1,0]
    assert tc.ra == 0

    tc = Computer("""\
            Register A: 0
            Register B: 29
            Register C: 0
            
            Program: 1,7""")
    tc.run()
    assert tc.rb == 26

    tc = Computer("""\
            Register A: 0
            Register B: 2024
            Register C: 43690
            
            Program: 4,0""")
    tc.run()
    assert tc.rb == 44354

    # main example
    tc = Computer("""\
            Register A: 729
            Register B: 0
            Register C: 0
            
            Program: 0,1,5,4,3,0""")
    tc.run()
    assert tc.output == [4,6,3,5,6,3,5,2,1,0]

    # part 2 example
    tc = Computer("""\
            Register A: 117440
            Register B: 0
            Register C: 0
            
            Program: 0,3,5,4,3,0""")
    tc.run()
    assert tc.output == [0,3,5,4,3,0]


def quick_run_code(a: int, prog: list[int]) -> list[int]:
    c = Computer(f"""\
        Register A: {a}
        Register B: 0
        Register C: 0
        
        Program: {",".join([str(i) for i in prog])}""")
    c.run()
    return c.output


def find_ra_for_output(program_code: list[int]) -> int:
    # state is number and next digit in output
    working_set: Deque[tuple[int,int]] = collections.deque([(0,1)])

    while working_set:
        num, exp = working_set.pop()
        target_output = program_code[-exp:]
        print(f'working on: {oct(num):>20}, e{exp}, target {target_output}')

        for i in range(8):
            ra = ( num << 3 ) | i
            out = quick_run_code(ra, program_code)
            # print(f'    testing : {oct(ra):20} -> {out}')
            if program_code == out:
                return ra
            elif target_output == out:
                working_set.appendleft((ra, exp+1))


def solve():
    example = """\
    Register A: 729
    Register B: 0
    Register C: 0
    
    Program: 0,1,5,4,3,0
    """
    input_text = get_input_data(2024, 17, example)

    test_cases()

    # part 1
    c = Computer(input_text)
    print(c)
    c.run()
    print(c)

    # part 2
    print("\nProgram Code:")
    c.prettyprint_program()

    program_code = [2,4,1,3,7,5,1,5,0,3,4,3,5,5,3,0]
    result = find_ra_for_output(program_code)
    print(f"found RegA input {oct(result)} = {result}")
