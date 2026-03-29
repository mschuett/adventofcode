use std::collections::{HashSet, VecDeque};
use crate::helper;

type Steps = u32;
type Point = (u8,u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node {
    x: u8,
    y: u8,
    size: u16,
    used: u16,
    avail: u16,
}
impl Node {
    fn new(line: &str) -> Node {
        let words = line.trim().split_whitespace().collect::<Vec<&str>>();
        let (x1,y1) = words[0]
            .strip_prefix("/dev/grid/node-").unwrap()
            .split_once('-').unwrap();
        let x = x1.strip_prefix('x').unwrap().parse::<u8>().unwrap();
        let y = y1.strip_prefix('y').unwrap().parse::<u8>().unwrap();
        let size = words[1].strip_suffix('T').unwrap().parse::<u16>().unwrap();
        let used = words[2].strip_suffix('T').unwrap().parse::<u16>().unwrap();
        let avail = words[3].strip_suffix('T').unwrap().parse::<u16>().unwrap();
        Node { x, y, size, used, avail }
    }
    fn fits_on(&self, other: &Node) -> bool {
        self.used > 0  // not empty
        && (self.x != other.x || self.y != other.y) // not the same node
        && self.used <= other.avail  // wold fit
    }
    fn pos(&self) -> Point {
        (self.x, self.y)
    }
    fn is_neighbor(&self, other: &Node) -> bool {
        (self.x == other.x && self.y.abs_diff(other.y) == 1)
        || (self.y == other.y && self.x.abs_diff(other.x) == 1)
    }
    fn move_data_to(&mut self, other: &mut Node) {
        other.used += self.used;
        other.avail -= self.used;
        self.avail += self.used;
        self.used -= self.used;
    }
}

fn parse_input(input_text: &str) -> Vec<Node> {
    let lines = input_text.lines().collect::<Vec<&str>>();
    lines[2..]
        .iter().map(|&line| {Node::new(line)})
        .collect()
}

fn viable_pairs(nodes: Vec<Node>) -> u16 {
    let mut viable_pairs: u16 = 0;
    for i in 0..nodes.len() {
        for j in i+1..nodes.len() {
            if nodes[i].fits_on(&nodes[j])
                || nodes[j].fits_on(&nodes[i]) {
                viable_pairs += 1;
            }
        }
    }
    viable_pairs
}

fn solve_part1(input_text: String) -> String {
    let nodes = parse_input(&input_text);
    viable_pairs(nodes).to_string()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Network {
    nodes: Vec<Vec<Node>>,
    x_size: u8,
    y_size: u8,
    special: Point,
}
impl Network {
    fn from_vec(src_nodes: Vec<Node>) -> Network {
        let x_size = src_nodes.iter().max_by(|&a, &b| a.x.cmp(&b.x)).unwrap().x+1;
        let y_size = src_nodes.iter().max_by(|&a, &b| a.y.cmp(&b.y)).unwrap().y+1;
        let special: Point = (x_size-1, 0);  // the data we have to move
        let mut nodes: Vec<Vec<Node>> = vec![];
        for y in 0..y_size {
            let mut row = src_nodes
                .iter()
                .filter(|&n| n.y == y)
                .map(|n| n.clone())
                .collect::<Vec<Node>>();
            row.sort_by(|a, b| a.x.cmp(&b.x));
            nodes.push(row);
        }
        Network {nodes, x_size, y_size, special}
    }
    fn prettypretty_print(&self) {
        // only to verify the given visualisation
        for y in 0..self.y_size {
            let row = &self.nodes[y as usize]
                .iter().map(|n| {
                let is_special = self.special == (n.x, n.y);
                format!("{}{:2}T/{:2}T{}",
                        if is_special {"["} else {" "},
                        n.used, n.size,
                        if is_special {"]"} else {" "})
            })
                .collect::<Vec<String>>()
                .join("--");
            println!("{}", row);
        }
    }
    fn get_pos(&self, p: &Point) -> &Node {
        self.nodes.get(p.1 as usize).unwrap().get(p.0 as usize).unwrap()
    }
    fn pretty_print(&self) -> String {
        // let special_size = self.nodes[self.special.1 as usize][self.special.0 as usize].size;
        let mut rows: Vec<String> = vec![];
        for y in 0..self.y_size {
            let row = &self.nodes[y as usize]
                .iter().map(|n| {
                let is_special = self.special == (n.x, n.y);
                if is_special { "G" }
                else if n.used == 0 { "_" }
                else if n.used >= 2*self.nodes[0][0].size { "#" }
                else { "." }
            })
                .collect::<Vec<&str>>()
                .join("");
            rows.push(row.clone());
        }
        rows.join("\n")
    }
    fn possible_moves(&self) -> Vec<(Point,Point)> {
        let mut viable_pairs: Vec<(Point, Point)> = vec![];
        // rows
        for y in 0..self.y_size {
            for x in 0..self.x_size-1 {
                let a = &self.nodes[y as usize][x as usize];
                let b = &self.nodes[y as usize][(x+1) as usize];
                if a.fits_on(b) { viable_pairs.push((a.pos(), b.pos()))}
                if b.fits_on(a) { viable_pairs.push((b.pos(), a.pos()))}
            }
        }
        // columns
        for x in 0..self.x_size {
            for y in 0..self.y_size-1 {
                let a = &self.nodes[y as usize][x as usize];
                let b = &self.nodes[(y+1) as usize][x as usize];
                if a.fits_on(b) { viable_pairs.push((a.pos(), b.pos()))}
                if b.fits_on(a) { viable_pairs.push((b.pos(), a.pos()))}
            }
        }
        viable_pairs
    }
    fn move_data(&mut self, from: Point, to: Point) {
        // not sure about this cloning, but the borrow-checker does not let me work on the items directly :(
        let mut src = self.nodes.get_mut(from.1 as usize).unwrap().get_mut(from.0 as usize).unwrap().clone();
        let mut dst = self.nodes.get_mut(to.1 as usize).unwrap().get_mut(to.0 as usize).unwrap().clone();
        src.move_data_to(&mut dst);
        self.nodes[from.1 as usize][from.0 as usize] = src.clone();
        self.nodes[to.1 as usize][to.0 as usize] = dst.clone();
        if from == self.special {
            self.special = to;
        }
    }
}

fn bfs_search(network_init: Network) -> u32 {
    // works for example, but not for real data :(
    let mut queue: VecDeque<(Steps, Network)> = VecDeque::new();
    queue.push_back((0, network_init.clone()));

    let mut visited: HashSet<Network> = HashSet::new();
    visited.insert(network_init);

    let mut iterations: u32 = 0;
    loop {
        if queue.is_empty() {
            println!("queue empty... failed to find a solution");
            return 0;
        }
        let (steps, network) = queue.pop_front().unwrap();
        let picture = network.pretty_print();
        if network.special == (0,0) {
            println!("found a solution in {} steps:\n{}", steps, picture);
            return steps;
        }

        iterations += 1;
        if iterations % 1024 == 0 {
            println!(
                "it {}, cur steps {}, visited {}, queue len {}",
                iterations,
                steps,
                visited.len(),
                queue.len()
            );
        }
        for (from, to) in network.possible_moves().iter() {
            let mut next = network.clone();
            next.move_data(*from, *to);
            if !visited.contains(&next) {
                visited.insert(next.clone());
                queue.push_back((steps+1, next));
            }
        }
    }
}

fn solve_part2(input_text: String) -> String {
    let nodes = parse_input(&input_text);
    let network = Network::from_vec(nodes);
    println!("{}", network.pretty_print());
    bfs_search(network).to_string()
}

pub fn solve() {
    let input_text = helper::fetch_cache_input_text(2016, 22)
            .expect("Could not fetch input");
    //let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Example: {}", solve_part2(example_text()));

    /* no implementation for Part 2
     * I failed to optimize the BFS and then found the manual solution on Reddit :-/
     */
}

fn example_text() -> String {
    String::from(
        "\
root@ebhq-gridcenter# df -h
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%
")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "7");
    }
}
