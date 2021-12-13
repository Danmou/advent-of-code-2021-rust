use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Copy, Clone)]
enum FoldType {
    HORIZONTAL,
    VERTICAL,
}

fn read_input() -> (HashSet<(usize, usize)>, Vec<(FoldType, usize)>) {
    let file = File::open("inputs/13.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut second_part = false;
    let mut points = HashSet::new();
    let mut folds = Vec::new();
    for line in br.lines() {
        let line_ = line.unwrap();
        if line_ == "" {
            second_part = true;
            continue;
        }
        if !second_part {
            let (x, y) = match line_.split(",").collect::<Vec<_>>()[..] {
                [x, y] => (x.parse().unwrap(), y.parse().unwrap()),
                _ => panic!(),
            };
            points.insert((x, y));
        } else {
            let (dir, loc) = match line_.split("=").collect::<Vec<_>>()[..] {
                [start, end] => (
                    if start.ends_with("x") {
                        FoldType::VERTICAL
                    } else {
                        FoldType::HORIZONTAL
                    },
                    end.parse::<usize>().unwrap(),
                ),
                _ => panic!(),
            };
            folds.push((dir, loc));
        }
    }

    (points, folds)
}

fn fold_points(
    points: &HashSet<(usize, usize)>,
    fold_dir: FoldType,
    loc: usize,
) -> HashSet<(usize, usize)> {
    let mut new_points = HashSet::new();
    for &(x, y) in points.iter() {
        if fold_dir == FoldType::VERTICAL {
            if x > loc {
                new_points.insert((loc - (x - loc), y));
            } else {
                new_points.insert((x, y));
            }
        } else {
            if y > loc {
                new_points.insert((x, loc - (y - loc)));
            } else {
                new_points.insert((x, y));
            }
        }
    }
    new_points
}

fn draw_points(points: &HashSet<(usize, usize)>) {
    for y in 0..15 {
        for x in 0..15 {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let (mut points, folds) = read_input();
    println!("\n{}\n", points.len());
    // draw_points(&points);
    for &(fold_dir, loc) in folds.iter() {
        points = fold_points(&points, fold_dir, loc);
        println!("\n{}\n", points.len());
        // draw_points(&points);

    }
}
