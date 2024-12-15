import textwrap
from .helper import get_input_data
from days.day06 import Coordinate
from days.day06 import Direction

class Robot(object):
    pos: Coordinate
    dirs: list[Direction]
    time: int

    def __init__(self, pos: Coordinate, input_text: str) -> None:
        self.pos = pos
        self.time = 0
        dirmap = {
            '^': Direction.N,
            '>': Direction.E,
            'v': Direction.S,
            '<': Direction.W,
        }
        self.dirs = [dirmap[char] for char in input_text if char in dirmap.keys()]

    def __repr__(self) -> str:
        return f'R(t {self.time}, pos {self.pos}, dir {self.dirs})'

    def cur_dir(self) -> Direction:
        return self.dirs[self.time]

class WarehouseMap(object):
    width: int
    height: int
    walls: set[Coordinate]
    boxes: set[Coordinate]
    robot: Robot

    def __init__(self, input_text: str) -> None:
        map_text, dirs_text = input_text.split("\n\n")
        lines = [l.strip() for l in map_text.strip().split('\n')]
        self.width = len(lines[0])
        self.height = len(lines)
        assert self.width == self.height
        assert all([len(l) == self.width for l in lines])
        self.walls = set()
        self.boxes = set()
        robot_pos: None|Coordinate = None

        for y in range(self.height):
            for x in range(self.width):
                pos = Coordinate(x,y)
                if lines[y][x] == '#':
                    self.walls.add(pos)
                elif lines[y][x] == 'O':
                    self.boxes.add(pos)
                elif lines[y][x] == '@':
                    robot_pos = pos
        if robot_pos is None:
            raise ValueError('Robot not found')
        self.robot = Robot(robot_pos, dirs_text)

    def __repr__(self) -> str:
        return f'{self.__class__.__name__}(width={self.width}, height={self.height})'

    def __str__(self) -> str:
        outstr = self.__repr__() + '\n'
        for y in range(self.height):
            for x in range(self.width):
                pos = Coordinate(x,y)
                if pos in self.walls:
                    outstr += '#'
                elif pos in self.boxes:
                    outstr += 'O'
                elif pos == self.robot.pos:
                    outstr += '@'
                else:
                    outstr += '.'
            outstr += "\n"
        return outstr

    def is_free(self, pos: Coordinate) -> bool:
        return pos not in self.walls and pos not in self.boxes

    def map_step(self):
        new_robo_pos = self.robot.pos.step(self.robot.cur_dir())
        if new_robo_pos in self.walls:
            pass  # no move
        elif new_robo_pos in self.boxes:
            # there may be multiple boxes, so find the end of the row
            new_box_pos = new_robo_pos
            while new_box_pos in self.boxes:
                new_box_pos = new_box_pos.step(self.robot.cur_dir())

            if new_box_pos in self.walls:
                pass  # cannot move anything
            else:
                # robot pushes box(es)
                self.boxes.remove(new_robo_pos)
                self.boxes.add(new_box_pos)
                self.robot.pos = new_robo_pos
        else:
            self.robot.pos = new_robo_pos
        self.robot.time += 1

    def sum_of_gps(self) -> int:
        acc = 0
        for box in self.boxes:
            acc += 100 * box.y + box.x
        return acc

class Box(object):
    pos: tuple[Coordinate, Coordinate]
    def __init__(self, pos: tuple[Coordinate, Coordinate]) -> None:
        assert pos[0].y == pos[1].y
        self.pos = pos
    def __eq__(self, other):
        return isinstance(other, Box) and self.pos == other.pos
    def __hash__(self):
        return hash(self.pos)
    def __str__(self):
        return f"({self.pos[0].x}/{self.pos[1].x},{self.pos[0].y})"


class DoubleWarehouseMap(WarehouseMap):
    bigboxes: dict[Coordinate, Box]

    def __init__(self, input_text: str) -> None:
        super().__init__(input_text)

        # resize
        self.width *= 2
        new_walls = set()
        for wall in self.walls:
            new_walls.add(Coordinate(wall.x*2, wall.y))
            new_walls.add(Coordinate(wall.x*2+1, wall.y))
        self.walls = new_walls

        bigboxes = {}
        new_boxes = set()
        for boxpos in self.boxes:
            pos1 = Coordinate(boxpos.x*2, boxpos.y)
            pos2 = Coordinate(boxpos.x*2+1, boxpos.y)
            new_boxes.add(pos1)
            new_boxes.add(pos2)

            box = Box((pos1, pos2))
            bigboxes[pos1] = box
            bigboxes[pos2] = box
        self.boxes = new_boxes  # not really needed, but keeps the class interface
        self.bigboxes = bigboxes

        self.robot.pos = Coordinate(self.robot.pos.x*2, self.robot.pos.y)

    def __str__(self) -> str:
        outstr = self.__repr__() + '\n'
        for y in range(self.height):
            for x in range(self.width):
                pos = Coordinate(x,y)
                if pos in self.walls:
                    outstr += '#'
                elif pos in self.bigboxes:
                    if self.bigboxes[pos].pos[0] == pos:
                        outstr += '['
                    elif self.bigboxes[pos].pos[1] == pos:
                        outstr += ']'
                    else:
                        raise ValueError('unexpected box')
                elif pos == self.robot.pos:
                    outstr += '@'
                else:
                    outstr += '.'
            outstr += "\n"
        return outstr

    def map_step(self):
        new_robo_pos = self.robot.pos.step(self.robot.cur_dir())
        if new_robo_pos in self.walls:
            pass  # no move
        elif self.is_free(new_robo_pos):
            self.robot.pos = new_robo_pos
        else:
            assert new_robo_pos in self.bigboxes
            # there may be a tree of multiple boxes, so check if all of them may move
            boxes_to_push = {self.bigboxes[new_robo_pos]}

            while True:  # we always have room or hit a wall, so this always breaks
                new_positions = set()
                for box in boxes_to_push:
                    for pos in box.pos:
                        new_positions.add(pos.step(self.robot.cur_dir()))
                assert new_positions

                # remove "internal" positions inside the boxes
                for box in boxes_to_push:
                    for pos in box.pos:
                        if pos in new_positions: new_positions.remove(pos)

                if all([self.is_free(pos) for pos in new_positions]):
                    stuck_on_wall = False
                    break
                elif any([pos in self.walls for pos in new_positions]):
                    stuck_on_wall = True
                    break
                else:  # other boxes
                    for pos in new_positions:
                        if pos in self.bigboxes:
                            boxes_to_push.add(self.bigboxes[pos])
                            # and loop
            if stuck_on_wall:
                pass  # cannot move anything
            else:
                # robot pushes box(es)
                # remove map positions
                for box in boxes_to_push:
                    self.boxes.remove(box.pos[0])
                    self.boxes.remove(box.pos[1])
                    del self.bigboxes[box.pos[0]]
                    del self.bigboxes[box.pos[1]]
                # move
                for box in boxes_to_push:
                    box.pos = (box.pos[0].step(self.robot.cur_dir()), box.pos[1].step(self.robot.cur_dir()))
                # re-add map positions
                for box in boxes_to_push:
                    self.bigboxes[box.pos[0]] = box
                    self.bigboxes[box.pos[1]] = box
                    self.boxes.add(box.pos[0])
                    self.boxes.add(box.pos[1])
                self.robot.pos = new_robo_pos
        self.robot.time += 1

    def sum_of_gps(self) -> int:
        acc = 0
        for box in set(self.bigboxes.values()):
            acc += 100 * box.pos[0].y + box.pos[0].x
        return acc



def solve():
    example = """\
    ##########
    #..O..O.O#
    #......O.#
    #.OO..O.O#
    #..O@..O.#
    #O#..O...#
    #O..O..O.#
    #.OO.O.OO#
    #....O...#
    ##########
    
    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    """
    # example = """\
    # ########
    # #..O.O.#
    # ##@.O..#
    # #...O..#
    # #.#.O..#
    # #...O..#
    # #......#
    # ########
    #
    # <^^>>>vv<v>>v<<
    # """
    input_text = get_input_data(2024, 15, example)

    # part 1
    m = WarehouseMap(textwrap.dedent(input_text))
    print(m)
    print(m.robot)
    while m.robot.time < len(m.robot.dirs):
        m.map_step()
    print(m)
    print(m.sum_of_gps())

    # part 2
    # input_text = """\
    # #######
    # #...#.#
    # #.....#
    # #..OO@#
    # #..O..#
    # #.....#
    # #######
    #
    # <vv<<^^<<^^
    # """
    m = DoubleWarehouseMap(textwrap.dedent(input_text))
    print(m)
    while m.robot.time < len(m.robot.dirs):
        m.map_step()
    print(m)
    print(m.sum_of_gps())
