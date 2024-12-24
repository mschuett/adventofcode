import collections
import itertools
import textwrap
from itertools import combinations
from typing import *
from .helper import get_input_data

class Gate(NamedTuple):
    src1: str
    op: str
    src2: str
    dst: str

class LogicDevice(object):
    wires: dict[str,bool]
    gates: set[Gate]
    formula: dict[str, Gate]
    gates_to_switch = [
            (Gate("x10", "AND", "y10", "z10"), Gate("htv", "XOR", "whd", "gpr")),
            (Gate("tkq", "XOR", "mvc", "ghp"), Gate("jtg", "OR", "trf", "z33")),
            (Gate("scj", "XOR", "ptd", "nks"), Gate("ptd", "AND", "scj", "z21")),
            (Gate("x39", "XOR", "y39", "cpm"), Gate("y39", "AND", "x39", "krs")),
        ]

    def __init__(self, input_data: str) -> None:
        self.wires = {}
        self.gates = set()
        self.formula = {}
        wiretext,gatetext = input_data.strip().split("\n\n")
        for line in wiretext.splitlines():
            name, value = line.split(":")
            self.wires[name] = bool(int(value))
        for line in gatetext.splitlines():
            word = line.split()
            assert word[3] == "->"
            gate = Gate(word[0], word[1], word[2], word[4])
            self.gates.add(gate)
            self.formula[gate.dst] = Gate(word[0], word[1], word[2], word[4])

    def eval_gates(self):
        need_more_iterations = True
        all_output_wires = [name.dst for name in self.gates if name.dst[0] == 'z']
        while need_more_iterations:
            for gate in self.gates:
                if gate.src1 not in self.wires or gate.src2 not in self.wires:
                    continue
                src1, src2 = self.wires[gate.src1], self.wires[gate.src2]
                dst = None
                match gate.op:
                    case "AND":
                        dst = src1 and src2
                    case "OR":
                        dst = src1 or src2
                    case "XOR":
                        dst = src1 ^ src2
                assert dst is not None
                self.wires[gate.dst] = dst
                # print(f"{gate.src1} {src1} {gate.op} {gate.src2} {src2} -> {gate.dst} = {dst}")
            need_more_iterations = any([(dst not in self.wires) for dst in all_output_wires])

    def get_output(self) -> int:
        wires = [wire for wire in sorted(self.wires.keys(), reverse=True) if wire[0] == 'z']
        values = [self.wires[name] for name in wires]
        num = int("".join([str(int(v)) for v in values]), base=2)
        # print(f"{wires} -> {values} -> {num}")
        return num

    def dot_graph(self) -> str:
        output = "digraph {\n"
        gates = list(self.gates)
        gates.sort(key=lambda x: x.dst)
        for i,gate in enumerate(gates):
            output += f'  {{ {gate.src1} {gate.src2} }} -> {gate.op}_{i}\n'
            output += f'  {gate.op}_{i} -> {gate.dst}\n'
        output += "}\n"
        return output

    def resolve_outputs(self, levels = 1):
        outputs = [gate for gate in self.gates if gate.dst[0] == 'z']
        outputs.sort(key=lambda x: x.dst)
        for gate in outputs:
            if gate.src1 in self.formula:
                src = self.formula[gate.src1]
                part1 = f"({src.src1} {src.op} {src.src2})"
            else:
                part1 = gate.src1
            if gate.src2 in self.formula:
                src = self.formula[gate.src2]
                part2 = f"({src.src1} {src.op} {src.src2})"
            else:
                part2 = gate.src2
            if "XOR" in part1:
                resolved = f"{gate.dst} = {part1} {gate.op} {part2}"
            else:
                resolved = f"{gate.dst} = {part2} {gate.op} {part1}"
            print(resolved)

    def switch_gates(self) -> None:
        for gate1,gate2 in self.gates_to_switch:
            self.gates.remove(gate1)
            self.gates.remove(gate2)
            self.gates.add(Gate(gate1.src1, gate1.op, gate1.src2, gate2.dst))
            self.gates.add(Gate(gate2.src1, gate2.op, gate2.src2, gate1.dst))

    def print_gates_to_switch(self) -> str:
        wires = []
        for gate1,gate2 in self.gates_to_switch:
            wires.append(gate1.dst)
            wires.append(gate2.dst)
        return ",".join(sorted(wires))

def part2_diagnostic_info(input_text: str):
    ld = LogicDevice(input_text)
    ld.switch_gates()
    wire_count = 45
    faulty_output_wires = []
    for x in range(wire_count+1):  # +1 to test 0 AND 0 = 0
        ld = LogicDevice(input_text)
        ld.switch_gates()
        for i in range(wire_count):
            ld.wires[f"x{i:02}"] = False
            ld.wires[f"y{i:02}"] = False
        ld.wires[f"x{x:02}"] = True
        ld.eval_gates()
        result = ld.get_output()
        result_string = f"{result:050b}"
        print(f"{result_string}")
        for i,char in enumerate(reversed(result_string)):
            if char != '0' and i != x: faulty_output_wires.append(i-1)

    if faulty_output_wires:
        print(f"faulty outputs: {faulty_output_wires}")
        print("source gates for those are...")
        faulty_output_gate_names = [f"z{n:02}" for n in faulty_output_wires]
        faulty_output_gates = [gate for gate in ld.gates if gate.dst in faulty_output_gate_names]
        sources = set()
        for gate in faulty_output_gates:
            print(gate)
            sources.add(gate.src1)
            sources.add(gate.src2)
        faulty_source_gates = [gate for gate in ld.gates if gate.dst in sources]
        for gate in faulty_source_gates:
            print(gate)
        ld.resolve_outputs()

def solve():
    example = """\
    x00: 1
    x01: 0
    x02: 1
    x03: 1
    x04: 0
    y00: 1
    y01: 1
    y02: 1
    y03: 1
    y04: 1
    
    ntg XOR fgs -> mjb
    y02 OR x01 -> tnw
    kwq OR kpj -> z05
    x00 OR x03 -> fst
    tgd XOR rvg -> z01
    vdt OR tnw -> bfw
    bfw AND frj -> z10
    ffh OR nrd -> bqk
    y00 AND y03 -> djm
    y03 OR y00 -> psh
    bqk OR frj -> z08
    tnw OR fst -> frj
    gnj AND tgd -> z11
    bfw XOR mjb -> z00
    x03 OR x00 -> vdt
    gnj AND wpb -> z02
    x04 AND y00 -> kjc
    djm OR pbm -> qhw
    nrd AND vdt -> hwm
    kjc AND fst -> rvg
    y04 OR y02 -> fgs
    y01 AND x02 -> pbm
    ntg OR kjc -> kwq
    psh XOR fgs -> tgd
    qhw XOR tgd -> z09
    pbm OR djm -> kpj
    x03 XOR y03 -> ffh
    x00 XOR y04 -> ntg
    bfw OR bqk -> z06
    nrd XOR fgs -> wpb
    frj XOR qhw -> z04
    bqk OR frj -> z07
    y03 OR x01 -> nrd
    hwm AND bqk -> z03
    tgd XOR rvg -> z12
    tnw OR pbm -> gnj
    """
    input_text = textwrap.dedent(get_input_data(2024, 24, example))

    ld = LogicDevice(input_text)
    print(ld)
    # part 1
    ld.eval_gates()
    print(ld.get_output())

    # part 2
    # I basically solved this manually.
    # I used part2_diagnostic_info() to identify 4 broken bits,
    # then the Graphviz/.dot output to identify the Gates to switch.
    ld.switch_gates()
    with open("day24.dot", "w") as f:
        f.write(ld.dot_graph())
    part2_diagnostic_info(input_text)
    print(ld.print_gates_to_switch())
