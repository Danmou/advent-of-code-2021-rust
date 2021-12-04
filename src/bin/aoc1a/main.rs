use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input() -> Vec<i64> {
    let file = File::open("inputs/1.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let res: Result<Vec<_>, _> = br.lines().map(|line| line.unwrap().parse()).collect();

    match res {
        Ok(res) => return res,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}


fn count_increases(nums: Vec<i64>) -> i64 {
    let mut count = 0;
    let mut last = nums[0];
    for num in nums[1..].iter() {
        if *num > last {
            count += 1;
        }
        last = *num;
    }
    count
}

fn main() {
    let input = read_input();
    let count = count_increases(input);
    println!("{}", count);
}
