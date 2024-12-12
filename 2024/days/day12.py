from .helper import get_input_data
from typing import List, Set, Dict, Iterable
from days.day08 import Coordinate

class FarmRegion(object):
    type: int
    plots: List[Coordinate]
    def __init__(self, plot_type: int, plots: List[Coordinate]):
        self.type = plot_type
        self.plots = plots
        self.plots.sort()
    def __repr__(self):
        return f"({chr(self.type)}, {self.plots})"
    def __str__(self):
        return f"region of {chr(self.type)}, with price {self.size()} * {self.perimeter_length()} = {self.size() * self.perimeter_length()}"

    def perimeter_length(self) -> int:
        l = 0
        for cur_plot in self.plots:
            for neighbour in [
                Coordinate(cur_plot.x, cur_plot.y-1),
                Coordinate(cur_plot.x-1, cur_plot.y),
                Coordinate(cur_plot.x, cur_plot.y+1),
                Coordinate(cur_plot.x+1, cur_plot.y),
            ]:
                if neighbour not in self.plots:
                    l += 1
        return l

    def size(self) -> int:
        return len(self.plots)

    def cost(self) -> int:
        return self.perimeter_length() * self.size()

    def sides_horizontal(self) -> int:
        # store the x value of all crossings into/out of the region
        crossings_in : List[List[int]] = []
        crossings_out: List[List[int]] = []
        for y in range(min([c.y for c in self.plots]), max([c.y for c in self.plots])+2):
            y_in = []
            y_out = []
            for x in range(min([c.x for c in self.plots]), max([c.x for c in self.plots])+2):
                left = Coordinate(x-1, y)
                right = Coordinate(x, y)
                if (left not in self.plots) and (right in self.plots):
                    y_in.append(x)
                elif (left in self.plots) and (right not in self.plots):
                    y_out.append(x)
            assert len(y_in) == len(y_out)
            crossings_in.append(y_in)
            crossings_out.append(y_out)
        # count sides, ignore those that continue in next y
        count = 0
        for y, line in enumerate(crossings_in):
            for x in line:
                if x not in crossings_in[y+1]:
                    count += 1
        for y, line in enumerate(crossings_out):
            for x in line:
                if x not in crossings_out[y+1]:
                    count += 1
        # print(f"sides_horizontal({chr(self.type)}): in {crossings_in} out {crossings_out} => {count} sides")
        return count

    def sides_vertical(self) -> int:
        # store the y value of all crossings into/out of the region
        crossings_in : List[List[int]] = []
        crossings_out: List[List[int]] = []
        for x in range(min([c.x for c in self.plots]), max([c.x for c in self.plots])+2):
            x_in = []
            x_out = []
            for y in range(min([c.y for c in self.plots]), max([c.y for c in self.plots])+2):
                up = Coordinate(x, y-1)
                down = Coordinate(x, y)
                if (up not in self.plots) and (down in self.plots):
                    x_in.append(y)
                elif (up in self.plots) and (down not in self.plots):
                    x_out.append(y)
            assert len(x_in) == len(x_out)
            crossings_in.append(x_in)
            crossings_out.append(x_out)
        # count sides, ignore those that continue in next y
        count = 0
        for x, line in enumerate(crossings_in):
            for y in line:
                if y not in crossings_in[x+1]:
                    count += 1
        for x, line in enumerate(crossings_out):
            for y in line:
                if y not in crossings_out[x+1]:
                    count += 1
        # print(f"sides_vertical({chr(self.type)}): in {crossings_in} out {crossings_out} => {count} sides")
        return count

    def sides(self) -> int:
        return self.sides_horizontal() + self.sides_vertical()

    def discount_cost(self) -> int:
        return self.sides() * self.size()


class FarmArea(object):
    width: int
    height: int
    regions: List[FarmRegion]
    plots: Dict[Coordinate, int]

    def __init__(self, input_text: str):
        lines = [l.strip() for l in input_text.strip().split('\n')]
        self.width = len(lines[0])
        self.height = len(lines)
        assert self.width == self.height
        assert all([len(l) == self.width for l in lines])
        self.regions = []
        self.plots = {}
        for y in range(self.height):
            for x in range(self.width):
                self.plots[Coordinate(x,y)] = ord(lines[y][x])
        assert len(self.plots.keys()) == self.height * self.width

    def __str__(self) -> str:
        outstr = ""
        for y in range(self.height):
            for x in range(self.width):
                outstr += chr(self.plots[Coordinate(x,y)])
            outstr += "\n"
        return outstr

    def get_plot_val(self, pos: Coordinate) -> int:
        assert self.in_area(pos)
        return self.plots[pos]

    def in_area(self, pos: Coordinate) -> bool:
        return (0 <= pos.x < self.width) and (0 <= pos.y < self.height)

    def sweep_region(self, starting_pos: Coordinate) -> FarmRegion:
        plot_type = self.get_plot_val(starting_pos)
        old_plots = set()
        new_plots = {starting_pos}
        while new_plots:
            next_new_plots = set()
            for cur_plot in new_plots:
                old_plots.add(cur_plot)
                for neighbour in [
                    Coordinate(cur_plot.x, cur_plot.y-1),
                    Coordinate(cur_plot.x-1, cur_plot.y),
                    Coordinate(cur_plot.x, cur_plot.y+1),
                    Coordinate(cur_plot.x+1, cur_plot.y),
                ]:
                    if self.in_area(neighbour) \
                        and self.get_plot_val(neighbour) == plot_type \
                            and neighbour not in old_plots \
                            and neighbour not in new_plots:
                        next_new_plots.add(neighbour)
            old_plots.union(new_plots)
            new_plots = next_new_plots
        return FarmRegion(plot_type, list(old_plots))

    def split_regions(self) -> None:
        unsorted_plots = set(self.plots.keys())
        assert len(unsorted_plots) == self.width * self.height
        while unsorted_plots:
            pivot_plot = unsorted_plots.pop()
            new_region = self.sweep_region(pivot_plot)
            for plot in new_region.plots:
                if plot != pivot_plot:
                    unsorted_plots.remove(plot)
            self.regions.append(new_region)

    def total_price(self) -> int:
        return sum([r.cost() for r in self.regions])

    def total_discount_price(self) -> int:
        return sum([r.discount_cost() for r in self.regions])


def solve():
    example = """\
    AAAA
    BBCD
    BBCC
    EEEC"""
    example = """\
    RRRRIICCFF
    RRRRIICCCF
    VVRRRCCFFF
    VVRCCCJFFF
    VVVVCJJCFE
    VVIVCCJJEE
    VVIIICJJEE
    MIIIIIJJEE
    MIIISIJEEE
    MMMISSJEEE"""
    input_text = get_input_data(2024, 12, example)

    # part 1
    a = FarmArea(input_text)
    print(a)
    a.split_regions()
    # for r in a.regions:
    #     print(r)
    print(f"total price: {a.total_price()}")

    # part 2
    print(f"discount price: {a.total_discount_price()}")
