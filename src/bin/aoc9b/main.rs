use colored::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<Vec<u8>> {
    let file = File::open("inputs/9.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut map = Vec::new();
    for line in br.lines() {
        let line_ = line.unwrap();
        map.push(
            line_
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect::<Vec<u8>>(),
        );
    }
    map
}

fn get_lowest_points(map: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    let mut points = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let val = map[i][j];
            if i > 0 && map[i - 1][j] <= val {
                continue;
            }
            if i < map.len() - 1 && map[i + 1][j] <= val {
                continue;
            }
            if j > 0 && map[i][j - 1] <= val {
                continue;
            }
            if j < map[i].len() - 1 && map[i][j + 1] <= val {
                continue;
            }
            points.insert((i, j));
        }
    }
    points
}

fn get_basin(map: &Vec<Vec<u8>>, point: &(usize, usize)) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut to_visit = vec![];
    to_visit.push(point.clone());
    while to_visit.len() > 0 {
        let (i, j) = to_visit.pop().unwrap();
        visited.insert((i, j));
        for (i_, j_) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if i == 0 && i_ < 0
                || i == map.len() - 1 && i_ > 0
                || j == 0 && j_ < 0
                || j == map[i].len() - 1 && j_ > 0
            {
                continue;
            }
            let i__ =  (i as i32 + i_) as usize;
            let j__ =  (j as i32 + j_) as usize;
            let p = (i__, j__);
            if map[i__][j__] == 9 {
                continue;
            }
            if !visited.contains(&p) {
                to_visit.push(p.clone());
            }
        }
    }
    visited
}

fn main() {
    let map = read_input();
    let points = get_lowest_points(&map);
    let mut basins = points.iter().map(|p| get_basin(&map, p)).collect::<Vec<_>>();
    basins.sort_by_key(|b| b.len());
    basins.reverse();

    let biggest = basins[0..3].iter().flatten().collect::<HashSet<_>>();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if points.contains(&(i, j)) {
                print!("{}", map[i][j].to_string().green());
            } else if biggest.contains(&(i, j)) {
                print!("{}", map[i][j].to_string().red());
            } else {
                print!("{}", map[i][j]);
            }
        }
        println!();
    }

    println!("{}", basins[0..3].iter().map(|b| b.len()).reduce(|a, b| a * b).unwrap())
}
