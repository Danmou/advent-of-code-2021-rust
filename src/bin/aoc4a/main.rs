use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct BingoBoard {
    numbers: [[u8; 5]; 5],
    hits: [[bool; 5]; 5],
    number_index: HashMap<u8, (usize, usize)>,
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            numbers: [[0u8; 5]; 5],
            hits: [[false; 5]; 5],
            number_index: HashMap::new(),
        }
    }

    fn set_numbers(&mut self, numbers: &[u8]) {
        if numbers.len() != 25 {
            panic!();
        }
        for (i, num) in numbers.iter().enumerate() {
            let row = i / 5;
            let col = i % 5;
            self.number_index.insert(*num, (row, col));
            self.numbers[row][col] = *num;
        }
    }

    fn mark_number(&mut self, num: u8) {
        match self.number_index.get(&num) {
            Some(&(row, col)) => self.hits[row][col] = true,
            _ => return,
        }
    }

    fn won(&self) -> bool {
        for i in 0..5 {
            if self.hits[i].iter().all(|&x| x){
                return true;
            }
            if self.hits.map(|row| row[i]).iter().all(|&x| x) {
                return true;
            }
        }
        false
    }

    fn score(&self) -> u32 {
        let mut sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.hits[row][col] {
                    sum += self.numbers[row][col] as u32;
                }
            }
        }
        sum
    }
}

fn read_input() -> (Vec<u8>, Vec<BingoBoard>) {
    let file = File::open("inputs/4.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut lines = br.lines();
    let draws: Vec<u8> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    let nums: Vec<u8> = lines
        .map(|line| {
            let l = line.unwrap();
            let ns: Vec<u8> = l.split_whitespace().map(|n| n.parse::<u8>().unwrap()).collect::<Vec<u8>>();
            ns
        })
        .flatten()
        .collect();

    let mut boards = vec![];

    for chunk in nums.chunks_exact(25) {
        let mut board = BingoBoard::new();
        board.set_numbers(chunk);
        boards.push(board);
    }

    (draws, boards)
}

fn simulate_bingo(draws: Vec<u8>, mut boards: Vec<BingoBoard>) -> u32 {
    for draw in draws {
        for board in boards.iter_mut() {
            board.mark_number(draw);
            if board.won() {
                return (draw as u32) * board.score();
            }
        }
    }
    panic!();
}

fn main() {
    let (draws, boards) = read_input();
    let score = simulate_bingo(draws, boards);
    println!("{}", score);
}
