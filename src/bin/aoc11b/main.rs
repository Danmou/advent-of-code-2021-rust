use ndarray::prelude::*;
use ndarray::Zip;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Array2<u8> {
    let file = File::open("inputs/11.txt");

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
    Array2::from_shape_vec(
        (map.len(), map[0].len()),
        map.into_iter().flatten().collect::<Vec<u8>>(),
    )
    .unwrap()
}

fn simulate_step(map: &mut Array2<u8>) -> bool {
    *map += 1;
    let mut flashed = map.mapv(|_| false);
    let all_flashed = loop {
        let new_flashed = map.mapv(|v| v > 9);
        let mut any = false;
        let mut all = true;
        for ((i, j), _) in flashed.indexed_iter() {
            if new_flashed[[i, j]] {
                if !flashed[[i, j]] {
                    any = true;
                    for i_ in [-1, 0, 1] {
                        for j_ in [-1, 0, 1] {
                            if i_ == 0 && j_ == 0
                                || i == 0 && i_ < 0
                                || i == map.shape()[0] - 1 && i_ > 0
                                || j == 0 && j_ < 0
                                || j == map.shape()[1] - 1 && j_ > 0
                            {
                                continue;
                            }
                            let i__ = (i as i32 + i_) as usize;
                            let j__ = (j as i32 + j_) as usize;
                            if !new_flashed[[i__, j__]] {
                                map[[i__, j__]] += 1;
                            }
                        }
                    }
                }
            } else {
                all = false;
            }
        }
        flashed = new_flashed;
        if !any {
            break all;
        }
    };

    Zip::from(&flashed).and(&mut *map).for_each(|&f, v| {
        if f {
            *v = 0;
        }
    });

    all_flashed
}

fn main() {
    let mut map = read_input();
    for step in 0..1000 {
        if simulate_step(&mut map) {
            println!("{}", step + 1);
            break;
        }
    }
}
