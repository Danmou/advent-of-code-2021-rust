use pathfinding::prelude::{absdiff, astar, Matrix};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Matrix<u8> {
    let file = File::open("inputs/15.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let matrix = Matrix::from_rows(br.lines().map(|l| {
        l.unwrap()
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect::<Vec<_>>()
    }))
    .unwrap();

    let mut big_matrix = Matrix::new(matrix.rows * 5, matrix.columns * 5, 0);
    for row in 0..5 {
        for col in 0..5 {
            let mut submat = matrix.clone();
            for val in submat.values_mut() {
                for _ in 0..(row + col) {
                    if *val == 9 {
                        *val = 1;
                    } else {
                        *val += 1;
                    }
                }
            }
            big_matrix.set_slice((row * matrix.rows, col * matrix.columns), &submat);
        }
    }

    big_matrix
}

fn get_shortest_path(map: &Matrix<u8>) -> u32 {
    let start = (0, 0);
    let goal = (map.rows as u32 - 1, map.columns as u32 - 1);
    astar(
        &start,
        |&(r, c)| {
            let mut v = Vec::new();
            if (r as usize) < map.rows - 1 {
                v.push((r + 1, c));
            }
            if r > 0 {
                v.push((r - 1, c));
            }
            if (c as usize) < map.columns - 1 {
                v.push((r, c + 1));
            }
            if c > 0 {
                v.push((r, c - 1));
            }
            v.into_iter()
                .map(|p| (p, map[(p.0 as usize, p.1 as usize)] as u32))
        },
        |&(r, c)| absdiff(r, goal.0) + absdiff(c, goal.1),
        |&p| p == goal,
    )
    .unwrap()
    .1
}

fn main() {
    let map = read_input();
    let cost = get_shortest_path(&map);
    println!("{:?}", cost);
}
