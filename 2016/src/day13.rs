use std::array::from_fn;
use std::collections::{HashSet, VecDeque};

type Point2d = (usize, usize);
type Steps = u32;
const SIZE: usize = 50;  // big enough for both parts

struct Maze {
    wall: [[bool; SIZE]; SIZE],
}
impl Maze {
    fn is_wall(x: u32, y: u32, favnum: u32) -> bool {
        let n: u32 = x*x + 3*x + 2*x*y + y + y*y + favnum;
        n.count_ones() % 2 != 0
    }
    fn new(favnum: u32) -> Maze {
        let wall: [[bool; SIZE]; SIZE] = from_fn(|y|
            from_fn(|x|
                Maze::is_wall(x as u32, y as u32, favnum)
            )
        );
        Maze { wall }
    }
    fn pretty_print(&self, max_xy: usize, visited: &HashSet<Point2d>) {
        let output = (0..max_xy).map(|y|
            (0..max_xy)
                .map(|x| if self.wall[y][x] { '#' } else { if visited.contains(&(x, y)) { 'o' } else { '.' }} )
                .collect::<String>()
        )
            .collect::<Vec<String>>();
        println!("{}", output.join("\n"));
    }
    fn gen_pos_neighbours(&self, pos: Point2d) -> Vec<Point2d> {
        let mut result: Vec<Point2d> = vec![];
        let (x, y) = pos;

        // all neighbour points, considering maze boundaries
        if y > 0      {result.push((  x, y-1))}  // N
        if x > 0      {result.push((x-1, y))}    // W
        if x < SIZE-1 {result.push((x+1, y))}    // E
        if y < SIZE-1 {result.push((  x, y+1))}  // S

        // remove points with walls
        result.retain(|pos| !self.wall[pos.1][pos.0]);
        result
    }

    fn bfs_path(&self, start: Point2d, end: Point2d) -> u32 {
        let mut queue: VecDeque<(Steps, Point2d)> = VecDeque::new();
        let mut visited: HashSet<Point2d> = HashSet::new();
        queue.push_back((0, start));

        let mut iterations: u32 = 0;
        loop {
            let (steps, pos) = queue
                .pop_front()
                .expect(format!("error, empty search queue after iteration {}", iterations).as_str());
            if pos == end {
                return steps;
            }
            visited.insert(pos);

            iterations += 1;
            if iterations % 1024 == 0 {
                println!(
                    "it {}, cur steps {}, visited {}, queue len {}, current: {:?}",
                    iterations,
                    steps,
                    visited.len(),
                    queue.len(),
                    pos
                );
                // self.pretty_print(40, &visited);
            }

            self.gen_pos_neighbours(pos).iter().for_each(|&pos| {
                if !visited.contains(&pos) && !queue.contains(&((steps + 1, pos))) {
                    queue.push_back((steps + 1, pos))
                }
            })
        }
    }

    fn reachable_positions(&self, start: Point2d, max_steps: u32) -> u32 {
        let mut queue: VecDeque<(Steps, Point2d)> = VecDeque::new();
        let mut visited: HashSet<Point2d> = HashSet::new();
        queue.push_back((0, start));

        let mut iterations: u32 = 0;
        loop {
            let (steps, pos) = queue
                .pop_front()
                .expect(format!("error, empty search queue after iteration {}", iterations).as_str());
            visited.insert(pos);
            if steps > max_steps {
                return visited.len() as u32;
            }

            iterations += 1;
            if iterations % 1024 == 0 {
                println!(
                    "it {}, cur steps {}, visited {}, queue len {}, current: {:?}",
                    iterations,
                    steps,
                    visited.len(),
                    queue.len(),
                    pos
                );
                self.pretty_print(40, &visited);
            }

            self.gen_pos_neighbours(pos).iter().for_each(|&pos| {
                if !visited.contains(&pos) && !queue.contains(&((steps + 1, pos))) {
                    queue.push_back((steps + 1, pos))
                }
            })
        }
    }
}

fn solve_example(input_text: String) -> String {
    let maze = Maze::new(10);
    maze.bfs_path((1,1), (7,4)).to_string()
}

fn solve_part1(input_text: String) -> String {
    let maze = Maze::new(input_text.parse().unwrap());
    maze.bfs_path((1,1), (31,39)).to_string()
}

fn solve_part2(input_text: String) -> String {
    let maze = Maze::new(input_text.parse().unwrap());
    let positions = maze.reachable_positions((1,1), 50) - 1;  // subtract start pos
    positions.to_string()
}

pub fn solve() {
    let input_text = String::from("1358");

    println!("Example: {}", solve_example(example_text()));
    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}

fn example_text() -> String {
    String::from("10")
}
