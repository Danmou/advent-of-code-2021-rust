use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Image {
    _image: HashSet<(i32, i32)>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    inverted: bool,
}

impl Image {
    fn new(inverted: bool) -> Image {
        Image {
            _image: HashSet::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            inverted,
        }
    }

    fn get(&self, x: i32, y: i32) -> bool {
        self._image.contains(&(x, y)) ^ self.inverted
    }

    fn insert(&mut self, x: i32, y: i32) {
        self.min_x = min(self.min_x, x);
        self.max_x = max(self.max_x, x);
        self.min_y = min(self.min_y, y);
        self.max_y = max(self.max_y, y);
        self._image.insert((x, y));
    }

    fn get_window(&self, x: i32, y: i32) -> usize {
        let mut value = 0;
        let mut bit = 8;
        for y_ in y - 1..=y + 1 {
            for x_ in x - 1..=x + 1 {
                if self.get(x_, y_) {
                    value |= 1 << bit;
                }
                bit -= 1;
            }
        }
        value
    }
}

fn read_input() -> (Vec<bool>, Image) {
    let file = File::open("inputs/20.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);
    let mut lines = br.lines();

    let mut algo = Vec::new();
    for c in lines.next().unwrap().unwrap().chars() {
        algo.push(match c {
            '#' => true,
            '.' => false,
            _ => panic!(),
        });
    }

    lines.next();
    let mut image = Image::new(false);
    for (row, line) in lines.enumerate() {
        for (col, c) in line.unwrap().chars().enumerate() {
            match c {
                '#' => image.insert(col as i32, row as i32),
                '.' => {}
                _ => panic!(),
            };
        }
    }

    (algo, image)
}

fn print_image(image: &Image) {
    for y in image.min_y..=image.max_y {
        for x in image.min_x..=image.max_x {
            print!("{}", if image.get(x, y) { "#" } else { "." });
        }
        println!();
    }
}

fn enhance_image(image: &Image, algo: &Vec<bool>) -> Image {
    let inverted = image.inverted ^ algo[0];
    let mut new = Image::new(inverted);
    for y in image.min_y - 20..=image.max_y + 20 {
        for x in image.min_x - 20..=image.max_x + 20 {
            if algo[image.get_window(x, y)] ^ inverted {
                new.insert(x, y);
            }
        }
    }
    new
}

fn main() {
    let (algo, mut image) = read_input();
    for i in 0..50 {
        println!("{}: {}", i, image._image.len());
        image = enhance_image(&image, &algo);
    }
    println!("{}", image._image.len())
}
