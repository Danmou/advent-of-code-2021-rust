use ndarray::prelude::*;
use num::Integer;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Range<T: Copy + Integer> {
    min: T,
    max: T,
}

impl<T: Copy + Integer> Range<T> {
    fn contains(&self, p: T) -> bool {
        p >= self.min && p < self.max
    }

    fn crop(&mut self, other: &Range<T>) {
        self.min = max(self.min, other.min);
        self.max = min(self.max, other.max);
    }

    fn size(&self) -> T {
        max(self.max - self.min, T::zero())
    }
}

#[derive(Debug, Clone, Copy)]
struct Cuboid<T: Copy + Integer> {
    x: Range<T>,
    y: Range<T>,
    z: Range<T>,
    value: bool,
}

impl<T: Copy + Integer> Cuboid<T> {
    fn contains(&self, (x, y, z): (T, T, T)) -> bool {
        self.x.contains(x) && self.y.contains(y) && self.z.contains(z)
    }

    fn crop(&mut self, x: Range<T>, y: Range<T>, z: Range<T>) {
        self.x.crop(&x);
        self.y.crop(&y);
        self.z.crop(&z);
    }

    fn size(&self) -> T {
        self.x.size() * self.y.size() * self.z.size()
    }
}

fn read_input() {
    let file = File::open("inputs/22.txt").unwrap();

    let br = BufReader::new(file);

    let mut regions = Vec::new();
    let mut all_values = HashSet::new();
    for line in br.lines() {
        let line_ = line.unwrap();
        let (value, rest) = match line_.split(" ").collect::<Vec<_>>()[..] {
            [val, rest] => (
                match val {
                    "on" => true,
                    "off" => false,
                    _ => panic!(),
                },
                rest.to_string(),
            ),
            _ => panic!(),
        };
        let mut region = match rest
            .split(",")
            .map(|s| {
                match s[2..]
                    .to_string()
                    .split("..")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>()[..]
                {
                    [a, b] => {
                        all_values.insert(a);
                        all_values.insert(b + 1);
                        Range { min: a, max: b + 1 }
                    }
                    _ => panic!(),
                }
            })
            .collect::<Vec<_>>()[..]
        {
            [x, y, z] => Cuboid { x, y, z, value },
            _ => panic!(),
        };
        region.crop(
            Range { min: -50, max: 51 },
            Range { min: -50, max: 51 },
            Range { min: -50, max: 51 },
        );
        regions.push(region);
    }

    all_values.insert(-50);
    all_values.insert(51);

    let mut values_sorted = all_values.into_iter().collect::<Vec<_>>();
    values_sorted.sort();
    let mut id_to_value = HashMap::new();
    let mut value_to_id = HashMap::new();
    for (i, v) in values_sorted.iter().enumerate() {
        id_to_value.insert(i as usize, *v);
        value_to_id.insert(*v, i as usize);
    }

    let regions_coded = regions
        .iter()
        .map(|r| Cuboid {
            x: Range {
                min: value_to_id[&r.x.min],
                max: value_to_id[&r.x.max],
            },
            y: Range {
                min: value_to_id[&r.y.min],
                max: value_to_id[&r.y.max],
            },
            z: Range {
                min: value_to_id[&r.z.min],
                max: value_to_id[&r.z.max],
            },
            value: r.value,
        })
        .collect::<Vec<_>>();

    println!("{}", value_to_id.len());
    println!("{}", value_to_id[&-50]);
    println!("{}", value_to_id[&51]);

    let size = value_to_id[&51] - value_to_id[&-50] + 1;
    let mut map = Array3::zeros((size, size, size));

    for region in regions_coded.iter() {
        for x in region.x.min..region.x.max {
            for y in region.y.min..region.y.max {
                for z in region.z.min..region.z.max {
                    let block_size = (id_to_value[&(x + 1)] - id_to_value[&x])
                        * (id_to_value[&(y + 1)] - id_to_value[&y])
                        * (id_to_value[&(z + 1)] - id_to_value[&z]);
                    println!("{}", block_size);
                    map[[
                        x - value_to_id[&-50],
                        y - value_to_id[&-50],
                        z - value_to_id[&-50],
                    ]] = if region.value { block_size as u64 } else { 0 };
                }
            }
        }
    }

    // let size = 101;
    // let mut map = Array3::zeros((size, size, size));
    //
    // for region in regions.iter() {
    //     for x in region.x.min..region.x.max {
    //         for y in region.y.min..region.y.max {
    //             for z in region.z.min..region.z.max {
    //                 let block_size = 1u8;
    //                 map[[
    //                     (x + 50) as usize,
    //                     (y + 50) as usize,
    //                     (z + 50) as usize,
    //                 ]] = if region.value {
    //                     block_size as u64
    //                 } else {
    //                     0
    //                 };
    //             }
    //         }
    //     }
    // }
    println!("{}", map.iter().sum::<u64>());
    // println!("{:?}", map.slice(s![58..=62,58..=62,58..=62]));
    // println!("{:?}", id_to_value);
    // println!("{:?}", map);
}

fn main() {
    read_input();
}
