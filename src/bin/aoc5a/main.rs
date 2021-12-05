use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn from_str(s: &str) -> Point {
        let parts: Vec<&str> = s.split(",").collect();
        if parts.len() != 2 {
            panic!();
        }
        Point {
            x: parts[0].trim().parse().unwrap(),
            y: parts[1].trim().parse().unwrap(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from_str(s: &str) -> Line {
        let parts: Vec<&str> = s.split("->").collect();
        if parts.len() != 2 {
            panic!();
        }
        Line {
            start: Point::from_str(parts[0].trim()),
            end: Point::from_str(parts[1].trim()),
        }
    }

    fn get_points(&self) -> Vec<Point> {
        let mut points = vec![];
        if self.start.x == self.end.x {
            let y_min = min(self.start.y, self.end.y);
            let y_max = max(self.start.y, self.end.y);
            for y in y_min..=y_max {
                points.push(Point { x: self.start.x, y });
            }
        } else if self.start.y == self.end.y {
            let x_min = min(self.start.x, self.end.x);
            let x_max = max(self.start.x, self.end.x);
            for x in x_min..=x_max {
                points.push(Point { x, y: self.start.y });
            }
        }
        points
    }
}

struct Map {
    counts: HashMap<Point, u32>,
}

impl Map {
    fn new() -> Map {
        Map {
            counts: HashMap::new(),
        }
    }

    fn add_point(&mut self, point: Point) {
        *self.counts.entry(point).or_insert(0) += 1;
    }

    fn add_line(&mut self, line: Line) {
        for point in line.get_points() {
            self.add_point(point);
        }
    }

    fn print(&self) {
        for y in 0..10 {
            for x in 0..10 {
                let count = self.counts.get(&Point {x, y}).or(Some(&0u32)).unwrap();
                if *count == 0 {
                    print!(". ");
                } else {
                    print!("{} ", count);
                }
            }
            println!();
        }
    }
}

fn read_input() -> Vec<Line> {
    let file = File::open("inputs/5.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    br.lines().map(|l| Line::from_str(&*l.unwrap())).collect()
}

fn get_num_overlaps(lines: Vec<Line>) -> u32 {
    let mut map = Map::new();
    for &line in lines.iter() {
        map.add_line(line.clone());
    }
    map.counts.values().filter(|&&v| v > 1).count() as u32
}

fn main() {
    let lines = read_input();
    let num = get_num_overlaps(lines);
    println!("{:?}", num);
}
