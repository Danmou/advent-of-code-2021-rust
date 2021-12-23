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
                    .collect::<Vec<i64>>()[..]
                {
                    [a, b] => Range { min: a, max: b + 1 },
                    _ => panic!(),
                }
            })
            .collect::<Vec<_>>()[..]
        {
            [x, y, z] => Cuboid { x, y, z, value },
            _ => panic!(),
        };
        regions.push(region);
    }

    let mut all_values_x = HashSet::new();
    let mut all_values_y = HashSet::new();
    let mut all_values_z = HashSet::new();
    for region in regions.iter() {
        all_values_x.insert(region.x.min);
        all_values_x.insert(region.x.max);
        all_values_y.insert(region.y.min);
        all_values_y.insert(region.y.max);
        all_values_z.insert(region.z.min);
        all_values_z.insert(region.z.max);
    }

    all_values_x.insert(-50);
    all_values_x.insert(51);
    all_values_y.insert(-50);
    all_values_y.insert(51);
    all_values_z.insert(-50);
    all_values_z.insert(51);

    let mut values_sorted_x = all_values_x.into_iter().collect::<Vec<_>>();
    values_sorted_x.sort();
    let mut id_to_value_x = HashMap::new();
    let mut value_to_id_x = HashMap::new();
    for (i, v) in values_sorted_x.iter().enumerate() {
        id_to_value_x.insert(i as usize, *v);
        value_to_id_x.insert(*v, i as usize);
    }
    let mut values_sorted_y = all_values_y.into_iter().collect::<Vec<_>>();
    values_sorted_y.sort();
    let mut id_to_value_y = HashMap::new();
    let mut value_to_id_y = HashMap::new();
    for (i, v) in values_sorted_y.iter().enumerate() {
        id_to_value_y.insert(i as usize, *v);
        value_to_id_y.insert(*v, i as usize);
    }
    let mut values_sorted_z = all_values_z.into_iter().collect::<Vec<_>>();
    values_sorted_z.sort();
    let mut id_to_value_z = HashMap::new();
    let mut value_to_id_z = HashMap::new();
    for (i, v) in values_sorted_z.iter().enumerate() {
        id_to_value_z.insert(i as usize, *v);
        value_to_id_z.insert(*v, i as usize);
    }

    let regions_coded = regions
        .iter()
        .map(|r| Cuboid {
            x: Range {
                min: value_to_id_x[&r.x.min],
                max: value_to_id_x[&r.x.max],
            },
            y: Range {
                min: value_to_id_y[&r.y.min],
                max: value_to_id_y[&r.y.max],
            },
            z: Range {
                min: value_to_id_z[&r.z.min],
                max: value_to_id_z[&r.z.max],
            },
            value: r.value,
        })
        .collect::<Vec<_>>();

    println!("{}", value_to_id_x.len());
    println!("{}", value_to_id_y.len());
    println!("{}", value_to_id_z.len());

    let mut map = Array3::zeros((
        value_to_id_x.len(),
        value_to_id_y.len(),
        value_to_id_z.len(),
    ));

    for region in regions_coded.iter() {
        for x in region.x.min..region.x.max {
            for y in region.y.min..region.y.max {
                for z in region.z.min..region.z.max {
                    // if (-50 > id_to_value_x[&x] || id_to_value_x[&x] > 50)
                    //     || (-50 > id_to_value_y[&y] || id_to_value_y[&y] > 50)
                    //     || (-50 > id_to_value_z[&z] || id_to_value_z[&z] > 50)
                    // {
                    //     continue;
                    // }
                    let block_size = (id_to_value_x[&(x + 1)] - id_to_value_x[&x])
                        * (id_to_value_y[&(y + 1)] - id_to_value_y[&y])
                        * (id_to_value_z[&(z + 1)] - id_to_value_z[&z]);
                    // println!("{}", block_size);
                    map[[x, y, z]] = if region.value { block_size as u64 } else { 0 };
                }
            }
        }
    }

    println!("{}", map.iter().sum::<u64>());
}

fn main() {
    read_input();
}
