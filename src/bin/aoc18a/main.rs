use std::cell::RefCell;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

struct Pair {
    a: Option<Box<Pair>>,
    b: Option<Box<Pair>>,
    val: Option<u32>,
}

impl Pair {
    fn new() -> Pair {
        Pair {
            a: None,
            b: None,
            val: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.val.is_some()
    }

    fn set_left(&mut self, a: Pair) {
        self.a = Some(Box::new(a));
    }

    fn set_right(&mut self, b: Pair) {
        self.b = Some(Box::new(b));
    }

    fn set_val(&mut self, val: u32) {
        self.val = Some(val);
    }

    fn get_left(&self) -> &Pair {
        self.a.as_ref().unwrap()
    }

    fn get_right(&self) -> &Pair {
        self.b.as_ref().unwrap()
    }

    fn get_left_mut(&mut self) -> &mut Pair {
        self.a.as_mut().unwrap()
    }

    fn get_right_mut(&mut self) -> &mut Pair {
        self.b.as_mut().unwrap()
    }

    fn get_val(&self) -> u32 {
        self.val.unwrap()
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_leaf() {
            write!(f, "{}", self.get_val())
        } else {
            write!(f, "[{},{}]", self.get_left(), self.get_right())
        }
    }
}

fn read_input() -> Vec<Pair> {
    let file = File::open("inputs/18.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let br = BufReader::new(file);

    let mut pairs = Vec::new();
    for line in br.lines() {
        let line_ = line.unwrap();
        let mut pair = Rc::new(RefCell::new(Pair::new()));
        {
            let mut stack = vec![Rc::clone(&pair)];
            for c in line_.chars() {
                let current = stack.last().unwrap();
                match c {
                    c if c.is_digit(10) => current.deref_mut().set_val(c.to_string().parse().unwrap()),
                    '[' => {
                        // let mut new = Pair::new();
                        // current.borrow_mut().set_left(new);
                        // let left = current.get_left_mut();
                        // stack.push(RefCell::new(left));
                    }
                    ',' => {}
                    ']' => {}
                    _ => panic!(),
                }
            }
        }
        pairs.push(pair.into_inner());
    }

    pairs
}

fn main() {
    let pairs = read_input();
    for p in pairs {
        println!("{}", p);
    }
    // println!(
    //     "{}",
    //     find_num_options(Target {
    //         x_min: 206,
    //         x_max: 250,
    //         y_min: -105,
    //         y_max: -57
    //     })
    // );
}
