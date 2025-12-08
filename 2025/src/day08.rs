use crate::helper;
use crate::coord3d::Coord3d;
use itertools::Itertools;

// make some types more readable
type Cluster = Vec<Coord3d>;
type Distance = (f32, Coord3d, Coord3d);

fn parse_input(input_text: &str) -> Vec<Coord3d> {
    let lines = input_text
        .lines()
        .collect::<Vec<&str>>();
    lines
        .into_iter()
        .map(Coord3d::from)
        .collect::<Vec<Coord3d>>()
}

fn precalc_distance_list(coords: &Vec<Coord3d>) -> Vec<Distance> {
    let mut shortest: Vec<Distance> = Vec::new();

    for v in coords.iter().combinations(2) {
        let (i,j) = (v[0], v[1]);
        if i == j { continue }
        let dist = i.dist(*j);
        shortest.push((dist, *i, *j));
    }
    shortest.sort_by(|a, b| a.partial_cmp(b).unwrap());
    shortest
}

fn to_clusters(coords: Vec<Coord3d>) -> Vec<Cluster> {
    let mut clusters: Vec<Cluster> = Vec::new();
    for c in &coords {
        let n = vec![*c];
        clusters.push(n);
    }
    clusters
}

fn solve_part1(input_text: String, n: usize) -> String {
    let coords = parse_input(&input_text);
    let distances = precalc_distance_list(&coords);
    let mut clusters = to_clusters(coords);

    for (i, d) in distances[..n].iter().enumerate() {
        if i % 500 == 0 {
            println!("{}: {} clusters", i, clusters.len());
        }
        let mut index1: usize = 0;
        let mut index2: usize = 0;
        for ci in 0..clusters.len() {
            if clusters[ci].contains(&d.1) {
                index1 = ci;
            }
            if clusters[ci].contains(&d.2) {
                index2 = ci;
            }
        }
        // both coords in the same cluster => nothing to do
        if index1 == index2 { continue; }
        if index2 < index1 { (index2, index1) = (index1, index2); }
        assert!(index1 < index2);
        // now we can swap_remove without changing anything at index1
        let c2: &mut Cluster = &mut clusters.swap_remove(index2);
        let c1: &mut Cluster = &mut clusters[index1];
        c1.append(c2);
    }

    clusters.sort_by_key(|c| c.len());
    clusters[..3].iter()
        .map(|c| c.len() as u32)
        .collect::<Vec<u32>>()
        .iter().product::<u32>()
        .to_string()
}


fn solve_part2(input_text: String) -> String {
    let coords = parse_input(&input_text);
    let distances = precalc_distance_list(&coords);
    let mut clusters = to_clusters(coords);
    let mut result = String::from("unknown");

    // part2
    for (i, d) in distances.iter().enumerate() {
        if i % 500 == 0 {
            println!("{}: {} clusters", i, clusters.len());
        }

        let mut index1: usize = 0;
        let mut index2: usize = 0;
        for ci in 0..clusters.len() {
            if clusters[ci].contains(&d.1) {
                index1 = ci;
            }
            if clusters[ci].contains(&d.2) {
                index2 = ci;
            }
        }
        // both coords in the same cluster => nothing to do
        if index1 == index2 { continue; }
        if index2 < index1 { (index2, index1) = (index1, index2); }
        assert!(index1 < index2);
        // now we can swap_remove without changing anything at index1
        let c2: &mut Cluster = &mut clusters.swap_remove(index2);
        let c1: &mut Cluster = &mut clusters[index1];
        c1.append(c2);

        if clusters.len() == 1 {
            println!("iteration {} connected last distance {:?}", i, d);
            // println!("result is {} * {} = {}", d.1.x(), d.2.x(), d.1.x() * d.2.x());
            result = (d.1.x() * d.2.x()).to_string();
            break;
        }
    }
    result
}


pub fn solve() {
    let (n, input_text) = (1000, helper::fetch_cache_input_text(2025, 8).expect("Could not fetch input"));
    let (_, _) = (10, example_text());

    println!("Part 1: {}", solve_part1(input_text.clone(), n));
    println!("Part 2: {}", solve_part2(input_text));
}


fn example_text() -> String {
    String::from("\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text(), 10);
        assert_eq!(result, "40");
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(example_text());
        assert_eq!(result, "25272");
    }
}
