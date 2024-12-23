import collections
import itertools
import textwrap
from itertools import combinations
from typing import *
from .helper import get_input_data


class LanMap(object):
    hosts: set[str]
    conns: set[tuple[str,str]]
    hostconns: dict[str, set[str]]

    def __init__(self, input_text: str) -> None:
        self.hosts = set()
        self.conns = set()
        self.hostconns = {}
        for line in input_text.splitlines():
            a,b = line.strip().split('-')
            self.hosts.add(a)
            self.hosts.add(b)
            self.conns.add((a,b))
            self.conns.add((b,a))
            self.hostconns[a] = self.hostconns.get(a,set()).union({b})
            self.hostconns[b] = self.hostconns.get(b,set()).union({a})
    def __repr__(self) -> str:
        return f'{self.__class__.__name__}({len(self.hosts)}, {len(self.conns)})'

    def find_cliques(self, minsize: int = 3, start_char: str = 't') -> Sequence[Iterable[str]]:
        def test_clique(cand: Iterable[str]) -> bool:
            for pair in combinations(cand, 2):
                if pair[0] != pair[1] and (pair[0],pair[1]) not in self.conns:
                    return False
            return True

        result = []
        for cand in itertools.combinations(self.hosts, minsize):
            if all([host[0] != start_char for host in cand]):
                continue
            if test_clique(cand):
                result.append(cand)
        return result

    def find_largest_clique(self, start: str|None = None) -> list[str]:
        if start is None:
            results = []
            for cand in self.hosts:
                results.append(self.find_largest_clique(cand))
            results.sort(key=len, reverse=True)
            return results[0]

        working_set: Deque[tuple[str,set[str],set[str]]] = collections.deque([(start, {start}, set())])
        largest = {start}
        while working_set:
            cur, result, visited = working_set.pop()
            for host in self.hostconns[cur]:
                if host in result or host in visited: continue
                if any([(host,other) not in self.conns for other in result]):
                    visited.add(host)
                else:
                    result.add(host)
                    working_set.append((host, result, visited))
                    if len(result) > len(largest):
                        largest = result
        return list(largest)

def part1(lm: LanMap) -> int:
    cliques = lm.find_cliques(3, 't')
    print(f"found {len(cliques)} cliques: {cliques}")
    count = 0
    for clique in cliques:
        if any([host[0] == 't' for host in clique]):
            count += 1
    return count

def part2(lm: LanMap) -> str:
    c = lm.find_largest_clique()
    c.sort()
    return ",".join(c)

def solve():
    example = """\
    kh-tc
    qp-kh
    de-cg
    ka-co
    yn-aq
    qp-ub
    cg-tb
    vc-aq
    tb-ka
    wh-tc
    yn-cg
    kh-ub
    ta-co
    de-co
    tc-td
    tb-wq
    wh-td
    ta-ka
    td-qp
    aq-cg
    wq-ub
    ub-vc
    de-ta
    wq-aq
    wq-vc
    wh-yn
    ka-de
    kh-ta
    co-tc
    wh-qp
    tb-vc
    td-yn
    """
    input_text = textwrap.dedent(get_input_data(2024, 23, example))

    lm = LanMap(input_text)
    print(lm)
    # part 1
    print(part1(lm))
    # part 2
    print(part2(lm))
