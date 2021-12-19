use rand::distributions::Uniform;
use rand::prelude::IteratorRandom;
use rand::Rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn rotate(&mut self, rotation: u8) {
        let (x, y, z) = match rotation {
            0 => (self.x, self.y, self.z),
            1 => (self.x, -self.y, -self.z),
            2 => (-self.x, self.y, -self.z),
            3 => (-self.x, -self.y, self.z),

            4 => (self.y, self.z, self.x),
            5 => (self.y, -self.z, -self.x),
            6 => (-self.y, self.z, -self.x),
            7 => (-self.y, -self.z, self.x),

            8 => (self.y, self.x, self.y),
            9 => (self.y, -self.x, -self.y),
            10 => (-self.y, self.x, -self.y),
            11 => (-self.y, -self.x, self.y),

            12 => (-self.z, -self.y, -self.x),
            13 => (-self.z, self.y, self.x),
            14 => (self.z, -self.y, self.x),
            15 => (self.z, self.y, -self.x),

            16 => (-self.y, -self.x, -self.z),
            17 => (-self.y, self.x, self.z),
            18 => (self.y, -self.x, self.z),
            19 => (self.y, self.x, -self.z),

            20 => (-self.x, -self.z, -self.y),
            21 => (-self.x, self.z, self.y),
            22 => (self.x, -self.z, self.y),
            23 => (self.x, self.z, -self.y),

            _ => panic!("invalid rotation"),
        };
        self.x = x;
        self.y = y;
        self.z = z;
    }

    fn rotated(&self, rotation: u8) -> Point {
        let mut p = self.clone();
        p.rotate(rotation);
        p
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

fn read_input() -> Vec<HashSet<Point>> {
    let file = File::open("inputs/19.txt").unwrap();

    let br = BufReader::new(file);

    let mut scans = Vec::new();
    let mut i = 0;
    for line in br.lines() {
        let line_ = line.unwrap();
        if line_ == "" {
            continue;
        }
        if line_.starts_with("---") {
            scans.push(HashSet::new());
            i = scans.len() - 1;
            continue;
        }
        let p = match line_
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>()[..]
        {
            [x, y, z] => Point { x, y, z },
            _ => panic!(),
        };
        scans[i].insert(p);
    }

    scans
}

fn match_scans(scans: &mut Vec<HashSet<Point>>) -> usize {
    let mut rng = rand::thread_rng();
    let rotation_dist = Uniform::from(0..24);
    'outer: while scans.len() > 1 {
        let rotation = rng.sample(rotation_dist);
        for i in 0..scans.len() {
            let &p1 = scans[i].iter().choose(&mut rng).unwrap();
            for j in i + 1..scans.len() {
                let p2 = scans[j].iter().choose(&mut rng).unwrap().rotated(rotation);
                let offset = p1 - p2;
                let count = count_matches(&scans[i], &scans[j], rotation, offset);
                if count >= 12 {
                    let scan_upd = scans
                        .remove(j)
                        .iter()
                        .map(|p| p.rotated(rotation) + offset)
                        .collect::<HashSet<_>>();
                    scans[i].extend(scan_upd);
                    println!(
                        "Match {}:{}, rotation {}, offset {:?}, {} left",
                        i,
                        j,
                        rotation,
                        offset,
                        scans.len() - 1
                    );
                    continue 'outer;
                }
            }
        }
    }
    scans[0].len()
}

fn count_matches(
    scan1: &HashSet<Point>,
    scan2: &HashSet<Point>,
    rotation: u8,
    offset: Point,
) -> u32 {
    let mut count = 0;
    for p in scan2 {
        if scan1.contains(&(p.rotated(rotation) + offset)) {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut scans = read_input();
    let num = match_scans(&mut scans);
    println!("{}", num)
}
