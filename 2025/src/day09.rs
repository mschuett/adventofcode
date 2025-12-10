use crate::helper;
use crate::coord2d::Point;
use itertools::Itertools;
use std::fs::File;
use std::io::Write;

fn parse_input(input_text: &str) -> Vec<Point> {
    input_text
        .lines()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(Point::from)
        .collect::<Vec<Point>>()
}

fn get_largest_area(coords: &Vec<Point>) -> (i64, Point, Point) {
    let mut largest: (i64, Point, Point) = (0, coords[0], coords[0]);

    for v in coords.iter().combinations(2) {
        let (i,j) = (v[0], v[1]);
        if i == j { continue }
        let area = i.area(*j);
        if area > largest.0 {
            largest = (area, *i, *j);
        }
    }
    largest
}

fn solve_part1(input_text: String) -> String {
    let coords = parse_input(&input_text);
    let (area, _i, _j) = get_largest_area(&coords);
    area.to_string()
}

// get the center of the rough circle
fn get_middle(coords: &Vec<Point>) -> Point {
    let mut max_x = coords[0].x;
    let mut max_y = coords[0].y;
    let mut min_x = coords[0].x;
    let mut min_y = coords[0].y;
    for p in coords {
        if p.x < min_x { min_x = p.x }
        if p.x > max_x { max_x = p.x }
        if p.y < min_y { min_y = p.y }
        if p.y > max_y { max_y = p.y }
    }
    let half_x = (max_x - min_x) / 2;
    let half_y = (max_y - min_y) / 2;
    Point{x: half_x, y: half_y}
}

// find area with one given pivot point as one fixed corner
fn get_largest_green_area(coords: &Vec<Point>, corner: &Point) -> (i64, Point, Point) {
    let mut largest: (i64, Point, Point) = (0, coords[0], coords[0]);

    for opp_corner in coords.iter() {
        // one secondary corner has the same y as the pivot, so it is always inside
        // now check if this fourth corner is inside the shape (it is not if we find any point with lower x/y)
        let secondary_corner = Point{x: corner.x, y: opp_corner.y};
        if coords.iter().any(|c|
            c.x.signum() == secondary_corner.x.signum() && c.x.abs() < secondary_corner.x.abs()
                && c.y.signum() == secondary_corner.y.signum() && c.y.abs() < secondary_corner.y.abs()) { continue }

        let area = corner.area(*opp_corner);
        if area > largest.0 {
            largest = (area, *corner, *opp_corner);
        }
    }
    largest
}

fn solve_part2(input_text: String) -> String {
    let coords1 = parse_input(&input_text);
    let middle = get_middle(&coords1);

    // move all coordinates to become relative to this middle
    // i _think_ that simplifies some value comparisons,
    // because then the x/y signum() always determines the quadrant
    let coords: Vec<Point> = coords1
        .iter().map(|p| Point{x: p.x - middle.x, y: p.y - middle.y} )
        .collect();

    // there are two "inner points" that create a horizontal gap
    // TODO: get them by algorithm
    // they are not the closest to the center,
    // maybe get the farthest distance from the previous/next point...
    let pivot_points = Vec::from([
        Point{x: 94543 - middle.x, y: 50265 - middle.y},
        Point{x: 94543 - middle.x, y: 48498 - middle.y}
    ]);

    // write file for debugging/graphing with gnuplot
    {
        let mut file = File::create("day9_shifted.txt").unwrap();
        for p in &coords {
            file.write_all(format!("{},{}\n", p.x, p.y).as_bytes()).unwrap();
        }
    }

    // now make the near-circle semi-convex, remove omit pivots and outlying points.
    // it is not mathmatically convex, as you can still draw lines that cross the border,
    // but convex enough for our purpose, because you cannot draw a rectangle across borders.
    //
    // with one exception!: we remove the pivot points, so we have to consider them later
    // hence we immediately split the circle into an upper and a lower half-circle
    let mut upper_half_circle: Vec<Point> = Vec::new();
    let mut lower_half_circle: Vec<Point> = Vec::new();
    for i in 0..coords.len() {
        let cur_point = coords[i];
        if cur_point == pivot_points[0] { continue }
        if cur_point == pivot_points[1] { continue }

        // check if there is another point closer to the center
        // seems like I have to split this into four quadrants :-/
        let is_outlyer = coords.iter()
            .any(|other|
                *other != cur_point
                && *other != pivot_points[0]
                && *other != pivot_points[1]
                && cur_point.x.signum() == other.x.signum()
                && cur_point.y.signum() == other.y.signum()
                && cur_point.x.abs() > other.x.abs() && cur_point.y.abs() > other.y.abs()
            );
        if !is_outlyer && cur_point.y >= pivot_points[0].y {
            upper_half_circle.push(cur_point);
        }
        if !is_outlyer && cur_point.y <= pivot_points[1].y {
            lower_half_circle.push(cur_point);
        }
    }
    // write file for debugging/graphing with gnuplot
    {
        let mut file = File::create("day9_filtered.txt").unwrap();
        for p in &lower_half_circle {
            file.write_all(format!("{},{}\n", p.x, p.y).as_bytes()).unwrap();
        }
        for p in &upper_half_circle {
            file.write_all(format!("{},{}\n", p.x, p.y).as_bytes()).unwrap();
        }
        println!("{} points after filter", upper_half_circle.len() + lower_half_circle.len());
    }

    // now try to find the largest rectangle, formed from a pivot point + two other points
    println!("Pivots are {} and {}", pivot_points[0], pivot_points[1]);
    let (area, _i, _j) = get_largest_green_area(&upper_half_circle, &pivot_points[0]);
    println!("Largest upper area is {} from points {:?} and {:?}", area, _i, _j);
    let (area2, _i, _j) = get_largest_green_area(&lower_half_circle, &pivot_points[1]);
    println!("Largest lower area is {} from points {:?} and {:?}", area2, _i, _j);

    if area > area2 {
        area.to_string()
    } else {
        area2.to_string()
    }
}


pub fn solve() {
    let input_text= helper::fetch_cache_input_text(2025, 9).expect("Could not fetch input");
    let _ = example_text();

    println!("Part 1: {}", solve_part1(input_text.clone()));
    println!("Part 2: {}", solve_part2(input_text));
}


fn example_text() -> String {
    String::from("\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_part1(example_text());
        assert_eq!(result, "50");
    }

    // #[test]
    // fn test_part2() {
    //     let result = solve_part2(example_text());
    //     assert_eq!(result, "24");
    // }
}
