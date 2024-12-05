import itertools

from .helper import get_input_data
from typing import List,Tuple,Union

def solve():
    example = """\
    47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47"""
    input_text = get_input_data(2024, 5, example)

    # part 1
    rule_text, print_run_text = input_text.split("\n\n")
    rules = [tuple([int(n) for n in l.strip().split('|')]) for l in rule_text.strip().splitlines()]
    print_runs: List[List[int]] = [[int(n) for n in l.strip().split(',')] for l in print_run_text.strip().splitlines()]

    def is_valid_print_run(rules: List[Tuple[int,...]], print_run: List[int]) -> bool:
        for rule in rules:
            first, second = rule
            if (first not in print_run) or (second not in print_run):
                continue
            if print_run.index(first) > print_run.index(second):
                # print(f'broken rule: {rule} in run {print_run}')
                return False
        return True

    valid_print_runs = []
    invalid_print_runs = []
    for pr in print_runs:
        if is_valid_print_run(rules,pr):
            valid_print_runs.append(pr)
        else:
            invalid_print_runs.append(pr)
    # print('valid: ', valid_print_runs)
    print('sum: ', sum([run[len(run)//2] for run in valid_print_runs]))

    # part 2
    def fix_print_run(rules: List[Tuple[int,...]], print_run: List[int]):
        def get_broken_rule(rules: List[Tuple[int,...]], print_run: List[int]) -> Union[None,Tuple[int,...]]:
            for rule in rules:
                first, second = rule
                if (first not in print_run) or (second not in print_run):
                    continue
                if print_run.index(first) > print_run.index(second):
                    # print(f'broken rule: {rule} in run {print_run}')
                    return rule
            return None
        while True:
            first,second = get_broken_rule(rules, print_run)
            # swap elements
            index_first = print_run.index(first)
            index_second = print_run.index(second)
            print_run[index_first] = second
            print_run[index_second] = first
            if is_valid_print_run(rules, print_run):
                return print_run

    corrected_print_runs = []
    for pr in invalid_print_runs:
        corrected_print_runs.append(fix_print_run(rules, pr))
    # print('corrected: ', corrected_print_runs)
    print('sum: ', sum([run[len(run)//2] for run in corrected_print_runs]))
