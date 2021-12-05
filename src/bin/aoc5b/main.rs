use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i16,
    y: i16,
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

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

#[derive(Debug, Clone)]
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
        let increment = Point {
            x: (self.end.x - self.start.x).signum(),
            y: (self.end.y - self.start.y).signum(),
        };
        let mut point = self.start.clone();
        while point != self.end {
            points.push(point.clone());
            point = point + increment.clone();
        }
        points.push(self.end.clone());
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
                let count = self.counts.get(&Point { x, y }).or(Some(&0u32)).unwrap();
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
    for line in lines.iter() {
        // println!("Adding line {:?}", line.clone());
        map.add_line(line.clone());
        // map.print();
    }
    map.counts.values().filter(|&&v| v > 1).count() as u32
}

fn main() {
    let lines = read_input();
    let num = get_num_overlaps(lines);
    println!("{:?}", num);
}
